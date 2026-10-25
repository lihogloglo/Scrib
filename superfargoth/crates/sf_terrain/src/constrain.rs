//! `constrain`: keeps objects authored on the source surface grounded.
//!
//! Under the footprints of grounded placements, the refined height blends
//! back to the source's own reference surface (interpolated the way the
//! source did, [`Triangulation`](crate::Triangulation)), fully inside the
//! footprint and fading out over a feather band around it.

use glam::Vec2;
use sf_coords::WorldPos;

use crate::source::{SourceMosaic, floor_i64, to_i64};
use crate::tile::{HeightTile, TILE_SAMPLES, TileCoord, sample_meters};

/// Default feather band around footprints, in meters.
pub const FOOTPRINT_FEATHER: f32 = 1.5;

/// The XZ footprint of a grounded placement: a rectangle rotated about +Y.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Footprint {
    /// Center; Y is ignored.
    pub center: WorldPos,
    /// Half size along the rectangle's local X and Z, in meters.
    pub half_extents: Vec2,
    /// Rotation about +Y in radians (right-handed, as `Quat::from_rotation_y`).
    pub yaw: f32,
}

/// Per-sample blend weights for one tile: 1 = source surface, 0 = refined.
#[derive(Clone, Debug, PartialEq)]
pub struct FootprintMask {
    pub coord: TileCoord,
    /// 513², row-major like [`HeightTile`]; values in [0, 1].
    pub weights: Vec<f32>,
}

impl FootprintMask {
    pub fn new(coord: TileCoord) -> Self {
        Self { coord, weights: vec![0.0; TILE_SAMPLES * TILE_SAMPLES] }
    }

    /// The mask of `footprints`, each feathered over `feather` meters.
    pub fn from_footprints(coord: TileCoord, footprints: &[Footprint], feather: f32) -> Self {
        let mut mask = Self::new(coord);
        for fp in footprints {
            mask.add(fp, feather);
        }
        mask
    }

    pub fn is_empty(&self) -> bool {
        self.weights.iter().all(|&w| w == 0.0)
    }

    /// Adds one footprint (union: the larger weight wins, so the order of
    /// footprints doesn't matter).
    pub fn add(&mut self, fp: &Footprint, feather: f32) {
        let center = fp.center.to_f64();
        let (sin, cos) = fp.yaw.sin_cos();
        // Half size of the rotated rectangle's axis-aligned bounds, plus feather.
        let reach = Vec2::new(
            fp.half_extents.x * cos.abs() + fp.half_extents.y * sin.abs(),
            fp.half_extents.x * sin.abs() + fp.half_extents.y * cos.abs(),
        ) + Vec2::splat(feather.max(0.0));
        let (gx0, gz0) = self.coord.global_sample(0, 0);
        let step = to_i64(1usize << self.coord.level);
        // Global sample range touched, clamped to the tile.
        let range = |c: f64, r: f32, g0: i64| {
            let lo = floor_i64((c - f64::from(r) + 128.0) * 2.0);
            let hi = floor_i64((c + f64::from(r) + 128.0) * 2.0) + 1;
            let last = g0 + to_i64(TILE_SAMPLES - 1) * step;
            let to_index = |g: i64| usize::try_from((g.clamp(g0, last) - g0) / step).expect("in tile");
            (to_index(lo), to_index(hi))
        };
        let (i0, i1) = range(center.x, reach.x, gx0);
        let (j0, j1) = range(center.z, reach.y, gz0);
        for j in j0..=j1 {
            let (_, gz) = self.coord.global_sample(0, j);
            #[allow(clippy::cast_possible_truncation)]
            let dz = (sample_meters(gz) - center.z) as f32;
            for i in i0..=i1 {
                let (gx, _) = self.coord.global_sample(i, 0);
                #[allow(clippy::cast_possible_truncation)]
                let dx = (sample_meters(gx) - center.x) as f32;
                // Into the rectangle's frame (inverse yaw).
                let local = Vec2::new(dx * cos - dz * sin, dx * sin + dz * cos);
                let outside = (local.abs() - fp.half_extents).max(Vec2::ZERO).length();
                let w = if outside == 0.0 {
                    1.0
                } else if outside >= feather {
                    0.0
                } else {
                    let t = 1.0 - outside / feather;
                    t * t * (3.0 - 2.0 * t)
                };
                let slot = &mut self.weights[j * TILE_SAMPLES + i];
                *slot = slot.max(w);
            }
        }
    }
}

/// Blends `tile` toward the source reference surface by `mask`. Weight 1
/// gives the reference height exactly.
pub fn constrain(tile: &mut HeightTile, src: &SourceMosaic, mask: &FootprintMask) {
    assert_eq!(tile.coord, mask.coord, "mask is for another tile");
    let Some((j0, j1, i0, i1)) = mask_bounds(mask) else { return };
    let lattice = src.heights.lattice;
    let corner = |i: usize, j: usize| {
        let (gx, gz) = tile.coord.global_sample(i, j);
        (lattice.coord(gx, 0), lattice.coord(gz, 1))
    };
    let (u0, v0) = corner(i0, j0);
    let (u1, v1) = corner(i1, j1);
    let min = [floor_i64(u0), floor_i64(v0)];
    let size = [to_usize_span(floor_i64(u1) - min[0] + 2), to_usize_span(floor_i64(v1) - min[1] + 2)];
    let window = src.heights.gather(min, size, src.fill_height);
    for j in j0..=j1 {
        for i in i0..=i1 {
            let w = mask.weights[j * TILE_SAMPLES + i];
            if w <= 0.0 {
                continue;
            }
            let (u, v) = corner(i, j);
            let reference = src.reference_in(&window, u, v);
            let h = &mut tile.heights[j * TILE_SAMPLES + i];
            *h = if w >= 1.0 { reference } else { *h + (reference - *h) * w };
        }
    }
}

fn to_usize_span(n: i64) -> usize {
    usize::try_from(n).expect("positive span")
}

/// Bounding rows and columns of the mask's nonzero weights.
fn mask_bounds(mask: &FootprintMask) -> Option<(usize, usize, usize, usize)> {
    let mut b: Option<(usize, usize, usize, usize)> = None;
    for (j, row) in mask.weights.as_chunks::<TILE_SAMPLES>().0.iter().enumerate() {
        let Some(first) = row.iter().position(|&w| w > 0.0) else { continue };
        let last = row.iter().rposition(|&w| w > 0.0).unwrap_or(first);
        b = Some(match b {
            None => (j, j, first, last),
            Some((j0, _, i0, i1)) => (j0, j, i0.min(first), i1.max(last)),
        });
    }
    b
}
