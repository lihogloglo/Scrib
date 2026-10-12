//! Layer maps (top-4 splat layers and weights) and tint maps.
//!
//! Level 0 resamples the source layer grid at 1 m: each sample blends the
//! texels around it with a tent filter, then keeps the four heaviest layers
//! (ties: lower id first) with weights quantized to sum to 255. Coarser
//! levels keep the dominant layers: each sample accumulates the 3×3 child
//! samples around it with (1, 2, 1) tent weights, in integers, and keeps
//! the top four again. Both read only global sample positions, so shared
//! edges agree across tiles.

use serde::{Deserialize, Serialize};

use crate::TerrainError;
use crate::source::{GridMosaic, SourceMosaic, Window, floor_i64, to_i64};
use crate::tile::{LAYER_SAMPLES, TILE_QUADS, TINT_SAMPLES, TileCoord};

/// A global terrain layer: index of a `TerrainLayer` in the bake's layer
/// table (and its texture array slice).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct LayerId(pub u32);

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LayerParams {
    /// Tent radius around texel centers in meters. The source texel spacing
    /// gives bilinear blending between neighboring texels.
    pub blend_radius: f32,
    /// Where no source texel has a layer.
    pub default_layer: LayerId,
}

/// One tile's layer map: 257² samples (1 m at level 0).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LayerMap {
    pub coord: TileCoord,
    /// Global layers used by this tile, sorted; slots index into it.
    pub palette: Vec<LayerId>,
    /// Four palette slots per sample, heaviest first.
    pub slots: Vec<[u8; 4]>,
    /// Weights of the first three slots; the fourth is `255 − sum`.
    pub weights: Vec<[u8; 3]>,
}

impl LayerMap {
    /// Sample (i, j) as (layer, weight) pairs; weights sum to 255. Unused
    /// slots repeat the first layer with weight 0.
    pub fn sample(&self, i: usize, j: usize) -> [(LayerId, u8); 4] {
        let k = j * LAYER_SAMPLES + i;
        let (s, w) = (self.slots[k], self.weights[k]);
        let w3 = 255 - w[0] - w[1] - w[2];
        let id = |n: usize| self.palette[usize::from(s[n])];
        [(id(0), w[0]), (id(1), w[1]), (id(2), w[2]), (id(3), w3)]
    }

    fn build(coord: TileCoord, samples: &[[(LayerId, u8); 4]]) -> Result<Self, TerrainError> {
        // Neighboring samples mostly repeat ids, and palettes are small:
        // skip repeats, then a linear check, instead of sorting every id.
        let mut palette: Vec<LayerId> = Vec::new();
        let mut last = None;
        for &(id, _) in samples.iter().flatten() {
            if last != Some(id) {
                last = Some(id);
                if !palette.contains(&id) {
                    palette.push(id);
                }
            }
        }
        palette.sort_unstable();
        if palette.len() > 256 {
            return Err(TerrainError::PaletteOverflow(coord));
        }
        let slot = |id: LayerId| u8::try_from(palette.binary_search(&id).expect("in palette")).expect("≤ 256");
        let slots = samples.iter().map(|s| s.map(|(id, _)| slot(id))).collect();
        let weights = samples.iter().map(|s| [s[0].1, s[1].1, s[2].1]).collect();
        Ok(Self { coord, palette, slots, weights })
    }
}

/// Keeps the four heaviest layers (ties: lower id), weights quantized to
/// sum to exactly 255 (cumulative rounding, so none goes negative).
fn top4(acc: &mut [(LayerId, f64)], default: LayerId) -> [(LayerId, u8); 4] {
    // Most samples sit inside one layer.
    if let [(id, w)] = *acc
        && w > 0.0
    {
        return [(id, 255), (id, 0), (id, 0), (id, 0)];
    }
    acc.sort_unstable_by(|a, b| b.1.total_cmp(&a.1).then(a.0.cmp(&b.0)));
    let kept = &acc[..acc.len().min(4)];
    let total: f64 = kept.iter().map(|e| e.1).sum();
    if kept.is_empty() || total <= 0.0 {
        return [(default, 255), (default, 0), (default, 0), (default, 0)];
    }
    let mut out = [(kept[0].0, 0u8); 4];
    let (mut cum, mut prev) = (0.0, 0u8);
    for (n, &(id, w)) in kept.iter().enumerate() {
        cum += w;
        // In [0, 255] by construction.
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let upto = if n + 1 == kept.len() { 255 } else { (cum / total * 255.0).round().min(255.0) as u8 };
        out[n] = (id, upto - prev);
        prev = upto;
    }
    // Rounding can swap near-equal weights; keep "heaviest first" exact.
    out[..kept.len()].sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
    out
}

fn add(acc: &mut Vec<(LayerId, f64)>, id: LayerId, w: f64) {
    match acc.iter_mut().find(|e| e.0 == id) {
        Some(e) => e.1 += w,
        None => acc.push((id, w)),
    }
}

/// Tent taps of one axis: sample `n` reads texels `starts[n] .. + taps`.
struct TentPlan {
    first: i64,
    span: usize,
    taps: usize,
    starts: Vec<usize>,
    weights: Vec<f32>,
}

impl TentPlan {
    /// `coord(n)`: continuous texel-center coordinate of sample `n`;
    /// `radius` in texels.
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::cast_precision_loss)]
    fn new(count: usize, radius: f64, coord: impl Fn(i64) -> f64) -> Self {
        let taps = 2 * radius.ceil() as usize;
        let mut starts = Vec::with_capacity(count);
        let mut weights = Vec::with_capacity(count * taps);
        for n in 0..to_i64(count) {
            let u = coord(n);
            let start = floor_i64(u - radius) + 1;
            for k in 0..to_i64(taps) {
                let d = ((start + k) as f64 - u).abs() / radius;
                weights.push((1.0 - d).max(0.0) as f32);
            }
            starts.push(start);
        }
        let first = starts[0];
        let span = (starts[count - 1] - first) as usize + taps;
        Self { first, span, taps, starts: starts.iter().map(|&s| (s - first) as usize).collect(), weights }
    }
}

/// The level-0 layer map of `coord`, or `None` if the source has no layers.
pub fn resample_layers(
    coord: TileCoord,
    src: &SourceMosaic,
    params: &LayerParams,
) -> Result<Option<LayerMap>, TerrainError> {
    assert_eq!(coord.level, 0, "coarser layer maps come from `downsample_layers`");
    let Some(mosaic) = &src.layers else { return Ok(None) };
    let lattice = mosaic.lattice;
    let radius = (f64::from(params.blend_radius) / lattice.spacing).max(1e-3);
    let (gx0, gz0) = coord.first_sample();
    // Samples every 2 level-0 indices (1 m); texel centers sit half a texel in.
    let px = TentPlan::new(LAYER_SAMPLES, radius, |n| lattice.coord(gx0 + 2 * n, 0) - 0.5);
    let pz = TentPlan::new(LAYER_SAMPLES, radius, |n| lattice.coord(gz0 + 2 * n, 1) - 0.5);
    let window: Window<Option<LayerId>> = GridMosaic::gather(mosaic, [px.first, pz.first], [px.span, pz.span], None);
    let mut samples = Vec::with_capacity(LAYER_SAMPLES * LAYER_SAMPLES);
    let mut acc = Vec::new();
    for j in 0..LAYER_SAMPLES {
        for i in 0..LAYER_SAMPLES {
            acc.clear();
            for b in 0..pz.taps {
                let wz = pz.weights[j * pz.taps + b];
                let row = (pz.starts[j] + b) * window.size[0];
                for a in 0..px.taps {
                    let w = wz * px.weights[i * px.taps + a];
                    if w > 0.0
                        && let Some(id) = window.data[row + px.starts[i] + a]
                    {
                        add(&mut acc, id, f64::from(w));
                    }
                }
            }
            samples.push(top4(&mut acc, params.default_layer));
        }
    }
    LayerMap::build(coord, &samples).map(Some)
}

/// Level-k layer map from level k−1 maps. `child` returns resident level
/// k−1 maps by coordinate; the 3×3 neighborhoods at the edges read the
/// neighbors of the four children too.
pub fn downsample_layers<'a>(
    coord: TileCoord,
    child: &dyn Fn(TileCoord) -> Option<&'a LayerMap>,
    params: &LayerParams,
) -> Result<LayerMap, TerrainError> {
    assert!(coord.level > 0, "level-0 layer maps come from `resample_layers`");
    let per = to_i64(LAYER_SAMPLES - 1);
    let level = coord.level - 1;
    // The 4×4 child tiles around this tile's 2×2 children.
    let (cx0, cz0) = (coord.x * 2 - 1, coord.z * 2 - 1);
    let mut grid: [Option<&LayerMap>; 16] = [None; 16];
    for (n, slot) in grid.iter_mut().enumerate() {
        let (dx, dz) = (i32::try_from(n % 4).expect("small"), i32::try_from(n / 4).expect("small"));
        *slot = child(TileCoord::new(level, cx0 + dx, cz0 + dz));
    }
    // Child-level global sample index -> (tile slot, local index), per axis.
    let locate = |g: i64, c0: i32| {
        let t = g.div_euclid(per);
        let slot = usize::try_from(t - i64::from(c0)).ok().filter(|&s| s < 4);
        (slot, usize::try_from(g - t * per).expect("in tile"))
    };
    // The three taps (slot, local index, tent weight) of every sample, per axis.
    let taps = |g0: i64, c0: i32| -> Vec<[(Option<usize>, usize, u8); 3]> {
        (0..to_i64(LAYER_SAMPLES))
            .map(|n| [-1i64, 0, 1].map(|d| {
                let (slot, local) = locate(g0 + 2 * n + d, c0);
                (slot, local, 2 - u8::from(d != 0))
            }))
            .collect()
    };
    let xs = taps(i64::from(coord.x) * per * 2, cx0);
    let zs = taps(i64::from(coord.z) * per * 2, cz0);
    let mut samples = Vec::with_capacity(LAYER_SAMPLES * LAYER_SAMPLES);
    let mut acc = Vec::new();
    for tz in &zs {
        for tx in &xs {
            acc.clear();
            for &(sz, lz, wz) in tz {
                for &(sx, lx, wx) in tx {
                    let (Some(sx), Some(sz)) = (sx, sz) else { continue };
                    let Some(map) = grid[sz * 4 + sx] else { continue };
                    let tent = f64::from(wx * wz);
                    for (id, w) in map.sample(lx, lz) {
                        if w > 0 {
                            add(&mut acc, id, tent * f64::from(w));
                        }
                    }
                }
            }
            samples.push(top4(&mut acc, params.default_layer));
        }
    }
    LayerMap::build(coord, &samples)
}

/// One tile's tint: 129² RGB samples (2 m at level 0).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TintMap {
    pub coord: TileCoord,
    pub rgb: Vec<[u8; 3]>,
}

/// Neutral tint where the source has none.
const WHITE: [u8; 3] = [255; 3];

/// The level-0 tint of `coord` (bilinear between source samples), or
/// `None` if the source has no tint.
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
pub fn resample_tint(coord: TileCoord, src: &SourceMosaic) -> Option<TintMap> {
    assert_eq!(coord.level, 0, "coarser tint maps come from `downsample_tint`");
    let mosaic = src.tint.as_ref()?;
    let lattice = mosaic.lattice;
    let (gx0, gz0) = coord.first_sample();
    let step = to_i64(TILE_QUADS / (TINT_SAMPLES - 1));
    let px = TentPlan::new(TINT_SAMPLES, 1.0, |n| lattice.coord(gx0 + step * n, 0));
    let pz = TentPlan::new(TINT_SAMPLES, 1.0, |n| lattice.coord(gz0 + step * n, 1));
    let window = mosaic.gather([px.first, pz.first], [px.span, pz.span], None);
    let mut rgb = Vec::with_capacity(TINT_SAMPLES * TINT_SAMPLES);
    for j in 0..TINT_SAMPLES {
        for i in 0..TINT_SAMPLES {
            let (mut sum, mut total) = ([0.0f32; 3], 0.0f32);
            for b in 0..pz.taps {
                let row = (pz.starts[j] + b) * window.size[0];
                for a in 0..px.taps {
                    let w = pz.weights[j * pz.taps + b] * px.weights[i * px.taps + a];
                    if w > 0.0
                        && let Some(c) = window.data[row + px.starts[i] + a]
                    {
                        for (s, v) in sum.iter_mut().zip(c) {
                            *s += w * f32::from(v);
                        }
                        total += w;
                    }
                }
            }
            rgb.push(if total > 0.0 { sum.map(|s| (s / total).round().clamp(0.0, 255.0) as u8) } else { WHITE });
        }
    }
    Some(TintMap { coord, rgb })
}

/// Level-k tint from its four children (`TileCoord::children` order), at
/// even indices like heights. Missing children read as white.
pub fn downsample_tint(coord: TileCoord, children: [Option<&TintMap>; 4]) -> TintMap {
    assert!(coord.level > 0, "level-0 tint comes from `resample_tint`");
    let half = (TINT_SAMPLES - 1) / 2;
    let pick = |i: usize| if i <= half { (0, i * 2) } else { (1, (i - half) * 2) };
    let mut rgb = vec![WHITE; TINT_SAMPLES * TINT_SAMPLES];
    for j in 0..TINT_SAMPLES {
        let (cz, sj) = pick(j);
        for i in 0..TINT_SAMPLES {
            let (cx, si) = pick(i);
            if let Some(c) = children[cz * 2 + cx] {
                rgb[j * TINT_SAMPLES + i] = c.rgb[sj * TINT_SAMPLES + si];
            }
        }
    }
    TintMap { coord, rgb }
}
