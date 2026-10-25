//! Shadow-height map (weather.md "Distant shadows", tier 2): the horizon of
//! a heightfield toward one light direction (Max 1988; Sloan and Cohen 2000),
//! reduced to one value per texel. A receiver at height `y` above texel `x`
//! is lit when `y > h_s(x)`, with
//! `h_s(x) = max_i (h(x + s_i·L̂xz) − s_i·tan(elevation))`, and its penumbra
//! widens with `d_occ`, the horizontal distance to the occluder that sets it.
//!
//! Consumers that can't march the height clipmap themselves (far-field
//! impostors and proxies) read this map: one fetch per fragment.

use glam::{Vec2, Vec3};
use sf_coords::WorldPos;

use crate::query::{TerrainHeights, WorldRect};

/// Value of `h_s` where nothing occludes (the light is above every sample).
pub const UNOCCLUDED: f32 = -1.0e6;

/// One map: `size`² texels of `texel` meters, texel (0, 0) centered on
/// `min` (world XZ, meters). `data[z * size + x] = [h_s, d_occ]`.
#[derive(Clone, Debug, PartialEq)]
pub struct ShadowHeights {
    pub size: usize,
    pub texel: f64,
    pub min: [f64; 2],
    pub data: Vec<[f32; 2]>,
}

/// March settings: `steps` exponentially spaced samples from `first` to
/// `reach` meters (horizontal) toward the light.
#[derive(Clone, Copy, Debug)]
pub struct ShadowMarch {
    pub steps: u32,
    pub first: f64,
    pub reach: f64,
}

impl Default for ShadowMarch {
    fn default() -> Self {
        Self { steps: 40, first: 4.0, reach: 6000.0 }
    }
}

/// `h_s` and `d_occ` at world XZ `p` toward `to_light` (unit, Y up).
/// `height` returns the terrain height at a world XZ, `None` off the map.
pub fn shadow_height(height: &impl Fn(f64, f64) -> Option<f32>, p: [f64; 2], to_light: [f64; 3], march: ShadowMarch) -> [f32; 2] {
    let lxz = to_light[0].hypot(to_light[2]);
    if to_light[1] <= 0.0 || lxz < 1e-4 {
        return [UNOCCLUDED, 0.0];
    }
    let dir = [to_light[0] / lxz, to_light[2] / lxz];
    let slope = to_light[1] / lxz;
    let ratio = (march.reach / march.first).powf(1.0 / f64::from(march.steps.max(2) - 1));
    let (mut best, mut occ) = (f64::from(UNOCCLUDED), 0.0);
    let mut s = march.first;
    for _ in 0..march.steps {
        if let Some(h) = height(p[0] + dir[0] * s, p[1] + dir[1] * s) {
            let hs = f64::from(h) - s * slope;
            if hs > best {
                best = hs;
                occ = s;
            }
        }
        s *= ratio;
    }
    #[allow(clippy::cast_possible_truncation)]
    [best as f32, occ as f32]
}

/// The whole map (callers run it off the main thread).
pub fn shadow_heights(
    height: &impl Fn(f64, f64) -> Option<f32>,
    min: [f64; 2],
    texel: f64,
    size: usize,
    to_light: [f64; 3],
    march: ShadowMarch,
) -> ShadowHeights {
    let row = |z: usize| -> Vec<[f32; 2]> {
        #[allow(clippy::cast_precision_loss)]
        let pz = min[1] + z as f64 * texel;
        (0..size)
            .map(|x| {
                #[allow(clippy::cast_precision_loss)]
                let px = min[0] + x as f64 * texel;
                shadow_height(height, [px, pz], to_light, march)
            })
            .collect()
    };
    let data = (0..size).flat_map(row).collect();
    ShadowHeights { size, texel, min, data }
}

/// Visibility of a receiver at height `y` given `[h_s, d_occ]`, with a
/// penumbra of `penumbra` meters per meter of occluder distance (the GPU
/// consumers mirror this).
pub fn visibility(hs: [f32; 2], y: f32, penumbra: f32) -> f32 {
    let w = hs[1] * penumbra + 0.5;
    let t = ((y - (hs[0] - w)) / (2.0 * w)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

/// The far shadow map around `center`: `size`² texels of `texel` meters,
/// centered on `center` (`min` is relative to it, meters), marched over the
/// resident heights resampled at `texel` spacing (the coarse heights are
/// what far shadows need, and a dense grid keeps the march cheap).
/// Unoccluded texels and the ones the light barely grazes are floored to
/// the ground minus two texels, so bilinear filtering between neighbors
/// stays within the terrain's range (no blend toward [`UNOCCLUDED`]).
/// Callers run it off the main thread (~0.1-0.2 s for 384² texels).
pub fn far_shadow_map(
    heights: &TerrainHeights,
    center: WorldPos,
    size: usize,
    texel: f64,
    to_light: [f64; 3],
    march: ShadowMarch,
) -> ShadowHeights {
    #[allow(clippy::cast_precision_loss)]
    let map_half = size as f64 * texel * 0.5;
    let half = map_half + march.reach;
    #[allow(clippy::cast_possible_truncation)]
    let rect = WorldRect {
        min: center.offset(Vec3::new(-half as f32, 0.0, -half as f32)),
        size: Vec2::splat((2.0 * half) as f32),
    };
    #[allow(clippy::cast_possible_truncation)]
    let spacing = texel as f32;
    let (nx, nz) = rect.sample_count(spacing);
    let mut grid = vec![0.0; nx * nz];
    heights.sample_rect(rect, spacing, &mut grid);
    // Bilinear over the grid, in center-relative meters.
    let height = |x: f64, z: f64| -> Option<f32> {
        let (gx, gz) = ((x + half) / texel, (z + half) / texel);
        if gx < 0.0 || gz < 0.0 {
            return None;
        }
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let (ix, iz) = (gx as usize, gz as usize);
        if ix + 1 >= nx || iz + 1 >= nz {
            return None;
        }
        #[allow(clippy::cast_possible_truncation, clippy::cast_precision_loss)]
        let (fx, fz) = ((gx - ix as f64) as f32, (gz - iz as f64) as f32);
        let at = |i: usize, j: usize| grid[j * nx + i];
        let top = at(ix, iz) + (at(ix + 1, iz) - at(ix, iz)) * fx;
        let bottom = at(ix, iz + 1) + (at(ix + 1, iz + 1) - at(ix, iz + 1)) * fx;
        let h = top + (bottom - top) * fz;
        (!h.is_nan()).then_some(h)
    };
    let min = -map_half + texel * 0.5;
    let mut map = shadow_heights(&height, [min, min], texel, size, to_light, march);
    #[allow(clippy::cast_possible_truncation)]
    let margin = (2.0 * texel) as f32;
    for (i, hs) in map.data.iter_mut().enumerate() {
        #[allow(clippy::cast_precision_loss)]
        let (px, pz) = (min + (i % size) as f64 * texel, min + (i / size) as f64 * texel);
        if let Some(ground) = height(px, pz) {
            hs[0] = hs[0].max(ground - margin);
        }
    }
    map
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A 100 m ridge along Z at x = 0, 20 m wide; flat ground elsewhere.
    #[allow(clippy::unnecessary_wraps, reason = "the height callback signature")]
    fn ridge(x: f64, _z: f64) -> Option<f32> {
        Some(if x.abs() < 10.0 { 100.0 } else { 0.0 })
    }

    /// Brute force: does the ray from (x, y) toward the light hit the ridge?
    fn brute_lit(x: f64, y: f64, to_light: [f64; 3]) -> bool {
        let lxz = to_light[0].hypot(to_light[2]);
        let slope = to_light[1] / lxz;
        let dir = to_light[0] / lxz;
        (1..60_000).all(|i| {
            let s = f64::from(i) * 0.1;
            f64::from(ridge(x + dir * s, 0.0).unwrap()) < y + s * slope
        })
    }

    #[test]
    fn ridge_shadow_matches_brute_force() {
        // Sun in +X at 10° elevation: the ridge shadows ~567 m to its west.
        let e = 10f64.to_radians();
        let l = [e.cos(), e.sin(), 0.0];
        let march = ShadowMarch { steps: 400, first: 1.0, reach: 6000.0 };
        for x in [-50.0, -200.0, -400.0, -540.0, -600.0, -900.0, 50.0] {
            let hs = shadow_height(&ridge, [x, 0.0], l, march);
            let lit = 0.0 > f64::from(hs[0]);
            assert_eq!(lit, brute_lit(x, 0.0, l), "x = {x}, h_s = {}", hs[0]);
        }
        // The occluder distance is to the ridge's near edge.
        let hs = shadow_height(&ridge, [-200.0, 0.0], l, march);
        assert!((f64::from(hs[1]) - 190.0).abs() < 10.0, "d_occ {}", hs[1]);
    }

    #[test]
    fn light_below_horizon_or_overhead_is_unoccluded() {
        let march = ShadowMarch::default();
        assert!(shadow_height(&ridge, [-50.0, 0.0], [0.0, 1.0, 0.0], march)[0] <= UNOCCLUDED);
        assert!(shadow_height(&ridge, [-50.0, 0.0], [1.0, -0.1, 0.0], march)[0] <= UNOCCLUDED);
    }

    #[test]
    fn map_layout_and_visibility() {
        let l = [0.9848, 0.1736, 0.0];
        let map = shadow_heights(&ridge, [-1000.0, -64.0], 16.0, 128, l, ShadowMarch::default());
        assert_eq!(map.data.len(), 128 * 128);
        // Texel x = 50 is at -200 m: shadowed at ground, lit 200 m up.
        let hs = map.data[10 * 128 + 50];
        assert!(visibility(hs, 0.0, 0.012) < 0.01);
        assert!(visibility(hs, 200.0, 0.012) > 0.99);
    }
}
