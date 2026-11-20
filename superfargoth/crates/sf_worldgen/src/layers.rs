//! **Ground layers of a guide, format v1** (spec `worldgen-adapter.md` "Same map at 10× (WG-X)" step 5, slice
//! WG-X3): a material layer grid an adapter writes into a guide folder, so a generated world draws the game's own
//! terrain layers instead of the fallback look. Generic: a layer is a record id string, hashed into an
//! `sf_terrain::LayerId` exactly as the terrain bake hashes its layer records (`sf_terrain::layer_id`), so the
//! ids index the layer arrays of any bake that has those records.
//!
//! A guide folder may hold:
//!
//! - `layers.json`: `{ "format": 1, "palette": [<layer record id>, ...], "default_layer": <id>, "global": <grid>,
//!   "stamps": [{ "stamp": <index into stamps.json>, ...<grid> }] }`, a grid being `{ "origin_m": [x, y],
//!   "dims": [w, h], "spacing_m", "blend_m", "offset" }`. `origin_m` is the guide-frame position (x east, y north,
//!   metres) of texel (0, 0)'s **north-west corner**; row 0 is the north edge, so texel (c, r) covers
//!   `x ∈ [x0 + c·s, x0 + (c + 1)·s]`, `y ∈ [y0 − (r + 1)·s, y0 − r·s]`. `blend_m` is the tent radius the layer
//!   maps blend texels over (`sf_terrain::LayerParams::blend_radius`). `offset` indexes `layers.u16`. The global
//!   grid covers the guide; a stamp's grid lies in the same (world) frame, used where that stamp's weight is
//!   over one half (`crate::stamps`). Extra keys are ignored.
//! - `layers.u16`: every grid's `w·h` texels (palette indices, `u16::MAX` = no layer: the default layer),
//!   little-endian, row-major, north up.
//!
//! **Tiles.** [`LayerField::layer_map`] builds a tile's `LayerMap` at any level from the global grid
//! (`sf_terrain::resample_layer_grid`) and, sample by sample, from the grid of the stamp whose weight there is
//! over one half. A pure function of the tile's coordinate: bitwise the same on any thread.
//!
//! **Macro look.** [`LayerField::with_means`] takes each layer's mean albedo (the bake's `LayerTable`) and builds
//! per grid a mean-albedo pyramid (texels averaged 2 × 2 per level, linear RGB in 10-bit words). The macro albedo
//! at a point is the bilinear of the finest pyramid level not finer than the macro sample spacing, in the same
//! global/stamp grid the layers use: what the terrain shader's tint (macro / layer mean) then cancels near the
//! camera. [`LayerField::gpu_words`] packs the pyramids for the GPU twin (`tiles.wgsl` `wg_layer_albedo`), which
//! evaluates the same f32 formula.

use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use sf_terrain::{LayerGrid, LayerId, LayerMap, LayerParams, LayerTable, TileCoord, layer_id, sample_meters};

use crate::stamps::{Stamp, StampSet};
use crate::{Error, Result};

#[cfg(test)]
#[path = "tests_layers.rs"]
mod tests;

/// Version of `layers.json`.
pub const LAYERS_FORMAT: u32 = 1;

/// No layer in a texel (`layers.u16`).
pub const NO_LAYER: u16 = u16::MAX;

/// A stamp weight above this takes the stamp's own layers (spec step 5: "inside them (w > 0.5)").
// tunables: const (format fact: the spec's stamp/global switch, the band's midpoint)
pub const STAMP_LAYER_WEIGHT: f32 = 0.5;

/// Mean albedo (linear) of a layer the table doesn't know, as the bake's tiles without layers.
// tunables: const (fallback: neutral grey for a palette id missing from the layer table, logged by the caller)
const UNKNOWN_ALBEDO: [f32; 3] = [0.18, 0.18, 0.18];

/// One texel grid (guide frame, module docs).
#[derive(Debug, Clone, PartialEq)]
pub struct GuideLayerGrid {
    /// Guide-frame metres (x east, y north) of texel (0, 0)'s north-west corner.
    pub origin_m: [f64; 2],
    /// `[w, h]` texels.
    pub dims: [usize; 2],
    pub spacing_m: f64,
    /// Tent radius of the layer blend, metres.
    pub blend_m: f64,
    /// Palette index per texel, row-major, north up; [`NO_LAYER`] = none.
    pub texels: Vec<u16>,
    /// Optional per-texel sRGB RGB8 tint (WG-X7c, e.g. Morrowind `VCLR`), row-major, north up.
    /// `None` = neutral white `[255, 255, 255]`. Stored in optional sidecar `layers.tint`.
    pub tint: Option<Vec<[u8; 3]>>,
}

impl GuideLayerGrid {
    fn validate(&self, palette: usize, what: &str) -> Result<()> {
        let bad = |m: String| Err(Error::Invalid(format!("layers {what}: {m}")));
        let [w, h] = self.dims;
        if w == 0 || h == 0 || w.checked_mul(h).is_none_or(|n| n >= 1 << 30) {
            return bad(format!("dims {w}x{h} out of range"));
        }
        if !(self.spacing_m.is_finite() && self.spacing_m > 0.0) {
            return bad("spacing_m must be > 0".into());
        }
        if !(self.blend_m.is_finite() && self.blend_m >= 0.0) {
            return bad("blend_m must be >= 0".into());
        }
        if !(self.origin_m[0].is_finite() && self.origin_m[1].is_finite()) {
            return bad("origin_m must be finite".into());
        }
        if self.texels.len() != w * h {
            return bad("texels length is not w·h".into());
        }
        if let Some(t) = &self.tint
            && t.len() != w * h
        {
            return bad("tint length is not w·h".into());
        }
        if let Some(t) = self.texels.iter().find(|&&t| t != NO_LAYER && usize::from(t) >= palette) {
            return bad(format!("texel {t} past the palette ({palette})"));
        }
        Ok(())
    }

    /// The rectangle, guide frame: `[x0, y0, x1, y1]` with x0 west, y0 south.
    pub fn rect_m(&self) -> [f64; 4] {
        let s = self.spacing_m;
        let [ox, oy] = self.origin_m;
        [ox, oy - self.dims[1] as f64 * s, ox + self.dims[0] as f64 * s, oy]
    }
}

/// The ground layers of a guide (format v1).
#[derive(Debug, Clone, PartialEq)]
pub struct GuideLayers {
    /// Layer record ids (`sf_terrain::layer_id` hashes them).
    pub palette: Vec<String>,
    /// Where no texel names a layer.
    pub default_layer: String,
    pub global: GuideLayerGrid,
    /// `(stamp index, grid)`, stamp indices distinct.
    pub stamps: Vec<(usize, GuideLayerGrid)>,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct GridV1 {
    origin_m: [f64; 2],
    dims: [usize; 2],
    spacing_m: f64,
    blend_m: f64,
    offset: u64,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct StampGridV1 {
    stamp: usize,
    #[serde(flatten)]
    grid: GridV1,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct FileV1 {
    format: u32,
    palette: Vec<String>,
    default_layer: String,
    global: GridV1,
    stamps: Vec<StampGridV1>,
}

impl GuideLayers {
    /// Reads `layers.json` + `layers.u16` from a guide folder; `None` when `layers.json` is absent.
    pub fn read(dir: &Path) -> Result<Option<Self>> {
        let json = dir.join("layers.json");
        if !json.exists() {
            return Ok(None);
        }
        let file: FileV1 = serde_json::from_reader(BufReader::new(File::open(json)?))?;
        if file.format != LAYERS_FORMAT {
            return Err(Error::Invalid(format!("layers format {} (v{LAYERS_FORMAT} expected)", file.format)));
        }
        let raw = std::fs::read(dir.join("layers.u16"))?;
        if raw.len() % 2 != 0 {
            return Err(Error::Invalid("layers.u16 length is odd".into()));
        }
        let all: Vec<u16> = raw.as_chunks::<2>().0.iter().map(|c| u16::from_le_bytes(*c)).collect();
        let tint_path = dir.join("layers.tint");
        let all_tint: Option<Vec<[u8; 3]>> = if tint_path.exists() {
            let tb = std::fs::read(&tint_path)?;
            if tb.len() != all.len() * 3 {
                return Err(Error::Invalid(format!(
                    "layers.tint has {} bytes, expected {}",
                    tb.len(),
                    all.len() * 3
                )));
            }
            Some(tb.as_chunks::<3>().0.to_vec())
        } else {
            None
        };
        let grid = |g: GridV1| -> Result<GuideLayerGrid> {
            let n = g.dims[0].saturating_mul(g.dims[1]);
            let off = usize::try_from(g.offset).map_err(|_| Error::Invalid("layers offset too large".into()))?;
            let end = off.checked_add(n).filter(|&e| e <= all.len());
            let Some(end) = end else { return Err(Error::Invalid("layers grid offset/dims past layers.u16".into())) };
            Ok(GuideLayerGrid {
                origin_m: g.origin_m,
                dims: g.dims,
                spacing_m: g.spacing_m,
                blend_m: g.blend_m,
                texels: all[off..end].to_vec(),
                tint: all_tint.as_ref().map(|t| t[off..end].to_vec()),
            })
        };
        let out = Self {
            palette: file.palette,
            default_layer: file.default_layer,
            global: grid(file.global)?,
            stamps: file.stamps.into_iter().map(|s| Ok((s.stamp, grid(s.grid)?))).collect::<Result<_>>()?,
        };
        out.validate()?;
        Ok(Some(out))
    }

    /// Writes `layers.json` + `layers.u16` (and optional `layers.tint` when any grid carries tint) into `dir`.
    pub fn write(&self, dir: &Path) -> Result<()> {
        self.validate()?;
        let has_tint = self.global.tint.is_some() || self.stamps.iter().any(|(_, g)| g.tint.is_some());
        let mut bytes = Vec::new();
        let mut tint_bytes = Vec::new();
        let mut put = |g: &GuideLayerGrid| {
            let offset = (bytes.len() / 2) as u64;
            for t in &g.texels {
                bytes.extend_from_slice(&t.to_le_bytes());
            }
            if has_tint {
                if let Some(ts) = &g.tint {
                    for rgb in ts {
                        tint_bytes.extend_from_slice(rgb);
                    }
                } else {
                    tint_bytes.resize(tint_bytes.len() + g.texels.len() * 3, 255);
                }
            }
            GridV1 { origin_m: g.origin_m, dims: g.dims, spacing_m: g.spacing_m, blend_m: g.blend_m, offset }
        };
        let global = put(&self.global);
        let stamps = self.stamps.iter().map(|(k, g)| StampGridV1 { stamp: *k, grid: put(g) }).collect();
        let file = FileV1 {
            format: LAYERS_FORMAT,
            palette: self.palette.clone(),
            default_layer: self.default_layer.clone(),
            global,
            stamps,
        };
        std::fs::create_dir_all(dir)?;
        std::fs::write(dir.join("layers.u16"), bytes)?;
        let tint_path = dir.join("layers.tint");
        if has_tint {
            std::fs::write(&tint_path, tint_bytes)?;
        } else {
            let _ = std::fs::remove_file(&tint_path);
        }
        std::fs::write(dir.join("layers.json"), serde_json::to_string_pretty(&file)?)?;
        Ok(())
    }

    fn validate(&self) -> Result<()> {
        if self.palette.len() >= usize::from(NO_LAYER) {
            return Err(Error::Invalid(format!("layers palette of {} entries", self.palette.len())));
        }
        self.global.validate(self.palette.len(), "global grid")?;
        let mut seen = std::collections::HashSet::new();
        for (k, g) in &self.stamps {
            if !seen.insert(*k) {
                return Err(Error::Invalid(format!("layers: two grids for stamp {k}")));
            }
            g.validate(self.palette.len(), &format!("stamp {k} grid"))?;
        }
        Ok(())
    }

    /// Texels of every grid.
    pub fn texel_count(&self) -> usize {
        self.global.texels.len() + self.stamps.iter().map(|(_, g)| g.texels.len()).sum::<usize>()
    }
}

/// One grid ready for tiles: its `sf_terrain` grid and blend.
struct FieldGrid {
    grid: LayerGrid,
    params: LayerParams,
}

/// A mean-albedo pyramid of one grid (module docs "Macro look").
#[derive(Debug, Clone, PartialEq)]
struct Pyramid {
    /// Base texel spacing, metres (f32: the GPU's value).
    spacing: f32,
    /// Per level: `[w, h]`, the f32 affine map `u = gx·a + cu`, `v = gz·a + cv` (level-0 global sample index →
    /// texel-centre coordinates), and the 10-bit words.
    levels: Vec<PyramidLevel>,
}

#[derive(Debug, Clone, PartialEq)]
struct PyramidLevel {
    dims: [usize; 2],
    a: f32,
    cu: f32,
    cv: f32,
    words: Vec<u32>,
}

/// Linear RGB 0..1 as one word of three 10-bit channels.
fn pack10(c: [f32; 3]) -> u32 {
    let q = |v: f32| (v.clamp(0.0, 1.0) * 1023.0).round() as u32;
    q(c[0]) | (q(c[1]) << 10) | (q(c[2]) << 20)
}

fn unpack10(w: u32) -> [f32; 3] {
    [(w & 1023) as f32 / 1023.0, ((w >> 10) & 1023) as f32 / 1023.0, ((w >> 20) & 1023) as f32 / 1023.0]
}

fn srgb_u8_to_linear(v: u8) -> f32 {
    let c = f32::from(v) / 255.0;
    if c <= 0.04045 { c / 12.92 } else { ((c + 0.055) / 1.055).powf(2.4) }
}

impl Pyramid {
    fn new(g: &GuideLayerGrid, mean: &dyn Fn(u16) -> [f32; 3]) -> Self {
        let [mut w, mut h] = g.dims;
        let mut cur: Vec<[f32; 3]> = match &g.tint {
            Some(tints) => g
                .texels
                .iter()
                .zip(tints)
                .map(|(&t, rgb)| {
                    let m = mean(t);
                    [
                        m[0] * srgb_u8_to_linear(rgb[0]),
                        m[1] * srgb_u8_to_linear(rgb[1]),
                        m[2] * srgb_u8_to_linear(rgb[2]),
                    ]
                })
                .collect(),
            None => g.texels.iter().map(|&t| mean(t)).collect(),
        };
        let mut levels = Vec::new();
        let mut s = g.spacing_m;
        loop {
            levels.push(PyramidLevel {
                dims: [w, h],
                a: (0.5 / s) as f32,
                cu: ((-128.0 - g.origin_m[0]) / s - 0.5) as f32,
                cv: ((g.origin_m[1] - 128.0) / s - 0.5) as f32,
                words: cur.iter().map(|&c| pack10(c)).collect(),
            });
            if w == 1 && h == 1 {
                break;
            }
            let (nw, nh) = (w.div_ceil(2), h.div_ceil(2));
            let mut next = vec![[0.0f32; 3]; nw * nh];
            for (k, out) in next.iter_mut().enumerate() {
                let (i, j) = (k % nw, k / nw);
                let (mut sum, mut n) = ([0.0f32; 3], 0.0f32);
                for (di, dj) in [(0, 0), (1, 0), (0, 1), (1, 1)] {
                    let (x, y) = (2 * i + di, 2 * j + dj);
                    if x < w && y < h {
                        let c = cur[y * w + x];
                        for a in 0..3 {
                            sum[a] += c[a];
                        }
                        n += 1.0;
                    }
                }
                *out = sum.map(|v| v / n);
            }
            cur = next;
            (w, h) = (nw, nh);
            s *= 2.0;
        }
        Self { spacing: g.spacing_m as f32, levels }
    }

    /// The pyramid level for macro level `level` (sample spacing `2^level` m): the coarsest whose texels are
    /// not wider than a sample (`tiles.wgsl` `wg_layer_albedo` picks the same).
    fn level_for(&self, level: u8) -> usize {
        let sp = (1u32 << level.min(30)) as f32;
        let mut m = 0usize;
        while m + 1 < self.levels.len() && self.spacing * (1u32 << (m + 1)) as f32 <= sp {
            m += 1;
        }
        m
    }

    /// Bilinear albedo (linear) at level-0 global sample (gx, gz), macro level `level`; f32 as the GPU.
    fn albedo(&self, gx: i64, gz: i64, level: u8) -> [f32; 3] {
        let l = &self.levels[self.level_for(level)];
        let [w, h] = l.dims;
        let u = (gx as f32 * l.a + l.cu).clamp(0.0, (w - 1) as f32);
        let v = (gz as f32 * l.a + l.cv).clamp(0.0, (h - 1) as f32);
        let (i, j) = ((u.floor() as usize).min(w - 1), (v.floor() as usize).min(h - 1));
        let (i1, j1) = ((i + 1).min(w - 1), (j + 1).min(h - 1));
        let (tx, ty) = (u - i as f32, v - j as f32);
        let (a, b) = (unpack10(l.words[j * w + i]), unpack10(l.words[j * w + i1]));
        let (c, d) = (unpack10(l.words[j1 * w + i]), unpack10(l.words[j1 * w + i1]));
        std::array::from_fn(|k| {
            let top = a[k] + (b[k] - a[k]) * tx;
            let bot = c[k] + (d[k] - c[k]) * tx;
            top + (bot - top) * ty
        })
    }

    fn word_count(&self) -> usize {
        2 + 6 * self.levels.len() + self.levels.iter().map(|l| l.words.len()).sum::<usize>()
    }

    /// The record at word `at`: mips, spacing, per level (offset, w, h, a, cu, cv), then the levels' words.
    fn gpu_words(&self, at: usize) -> Vec<u32> {
        let w32 = |v: usize| u32::try_from(v).expect("layer field offset fits u32");
        let mut d = vec![w32(self.levels.len()), self.spacing.to_bits()];
        let mut p = at + 2 + 6 * self.levels.len();
        for l in &self.levels {
            d.extend([w32(p), w32(l.dims[0]), w32(l.dims[1]), l.a.to_bits(), l.cu.to_bits(), l.cv.to_bits()]);
            p += l.words.len();
        }
        for l in &self.levels {
            d.extend(&l.words);
        }
        d
    }
}

/// A guide's layers ready for tiles ([`Self::layer_map`]) and, given the layers' mean albedos
/// ([`Self::with_means`]), for the macro look.
pub struct LayerField {
    layers: GuideLayers,
    ids: Vec<LayerId>,
    global: FieldGrid,
    /// By stamp index; `None` where a stamp has no grid.
    stamps: Vec<Option<FieldGrid>>,
    /// Global pyramid, then by stamp index.
    macros: Option<(Pyramid, Vec<Option<Pyramid>>)>,
}

impl LayerField {
    /// `stamp_count`: stamps of the guide (`stamps.json`); grids of other indices are an error.
    pub fn new(layers: GuideLayers, stamp_count: usize) -> Result<Self> {
        let ids: Vec<LayerId> = layers.palette.iter().map(|r| layer_id(r)).collect();
        let default_layer = layer_id(&layers.default_layer);
        let field = |g: &GuideLayerGrid| -> Result<FieldGrid> {
            let texels = g.texels.iter().map(|&t| (t != NO_LAYER).then(|| ids[usize::from(t)])).collect();
            // Engine frame: X = x, Z = −y; the north-west corner is the minimum (X, Z).
            let grid = LayerGrid::new([g.origin_m[0], -g.origin_m[1]], g.spacing_m as f32, g.dims, texels)
                .map_err(|e| Error::Invalid(format!("layers grid: {e}")))?;
            Ok(FieldGrid { grid, params: LayerParams { blend_radius: g.blend_m as f32, default_layer } })
        };
        let global = field(&layers.global)?;
        let mut stamps: Vec<Option<FieldGrid>> = (0..stamp_count).map(|_| None).collect();
        for (k, g) in &layers.stamps {
            let slot = stamps
                .get_mut(*k)
                .ok_or_else(|| Error::Invalid(format!("layers: grid for stamp {k}, the guide has {stamp_count} stamps")))?;
            *slot = Some(field(g)?);
        }
        Ok(Self { layers, ids, global, stamps, macros: None })
    }

    pub fn layers(&self) -> &GuideLayers {
        &self.layers
    }

    /// The palette's layer ids (`sf_terrain::layer_id` of each record).
    pub fn ids(&self) -> &[LayerId] {
        &self.ids
    }

    /// The macro pyramids from each layer's mean albedo in `table` (linear); returns the palette entries the
    /// table doesn't know (drawn neutral grey).
    pub fn with_means(&mut self, table: &LayerTable) -> Vec<String> {
        let by_id: std::collections::HashMap<LayerId, [f32; 3]> = table.layers.iter().map(|l| (l.id, l.mean_albedo)).collect();
        let mut missing = Vec::new();
        let mut means: Vec<[f32; 3]> = Vec::with_capacity(self.ids.len());
        for (id, rec) in self.ids.iter().zip(&self.layers.palette) {
            means.push(by_id.get(id).copied().unwrap_or_else(|| {
                missing.push(rec.clone());
                UNKNOWN_ALBEDO
            }));
        }
        let fill = by_id.get(&layer_id(&self.layers.default_layer)).copied().unwrap_or(UNKNOWN_ALBEDO);
        let mean = |t: u16| if t == NO_LAYER { fill } else { means[usize::from(t)] };
        let global = Pyramid::new(&self.layers.global, &mean);
        let mut stamps: Vec<Option<Pyramid>> = (0..self.stamps.len()).map(|_| None).collect();
        for (k, g) in &self.layers.stamps {
            stamps[*k] = Some(Pyramid::new(g, &mean));
        }
        self.macros = Some((global, stamps));
        missing
    }

    /// Whether [`Self::with_means`] ran (the macro look comes from the layers).
    pub fn has_macro(&self) -> bool {
        self.macros.is_some()
    }

    /// Stamps (indices into `set`) whose rectangle meets the guide-frame rectangle `r` ([x0, y0, x1, y1]): the
    /// only ones whose weight can be over one half there.
    fn candidates(set: &StampSet, r: [f64; 4]) -> Vec<usize> {
        set.stamps
            .iter()
            .enumerate()
            .filter_map(|(k, s)| {
                let g = s.rect_m();
                (g[0] <= r[2] && g[2] >= r[0] && g[1] <= r[3] && g[3] >= r[1]).then_some(k)
            })
            .collect()
    }

    /// Tile `c`'s rectangle in the guide frame.
    fn tile_rect(c: TileCoord) -> [f64; 4] {
        let (gx0, gz0) = c.first_sample();
        let span = (sf_terrain::TILE_QUADS as i64) << c.level;
        let (x0, x1) = (sample_meters(gx0), sample_meters(gx0 + span));
        let (z0, z1) = (sample_meters(gz0), sample_meters(gz0 + span));
        [x0, -z1, x1, -z0]
    }

    /// The layer map of tile `c` (any level): the global grid's, and where a stamp's weight is over
    /// [`STAMP_LAYER_WEIGHT`] that stamp's grid's (module docs "Tiles").
    pub fn layer_map(&self, c: TileCoord, stamps: Option<&StampSet>) -> Result<LayerMap> {
        let err = |e: sf_terrain::TerrainError| Error::Invalid(format!("layer map {c:?}: {e}"));
        let global = sf_terrain::resample_layer_grid(c, &self.global.grid, &self.global.params).map_err(err)?;
        let Some(set) = stamps else { return Ok(global) };
        let cands = Self::candidates(set, Self::tile_rect(c));
        if cands.iter().all(|&k| self.stamps.get(k).is_none_or(Option::is_none)) {
            return Ok(global);
        }
        let maps: Vec<Option<LayerMap>> = cands
            .iter()
            .map(|&k| {
                self.stamps.get(k).and_then(Option::as_ref).map(|g| sf_terrain::resample_layer_grid(c, &g.grid, &g.params)).transpose()
            })
            .collect::<std::result::Result<_, _>>()
            .map_err(err)?;
        let n = sf_terrain::LAYER_SAMPLES;
        let mut samples = Vec::with_capacity(n * n);
        let mut any = false;
        for j in 0..n {
            for i in 0..n {
                let (gx, gz) = c.global_sample(2 * i, 2 * j);
                let pick = pick_stamp(&set.stamps, &cands, sample_meters(gx), -sample_meters(gz));
                samples.push(match pick.and_then(|p| maps[p].as_ref()) {
                    Some(m) => {
                        any = true;
                        m.sample(i, j)
                    }
                    None => global.sample(i, j),
                });
            }
        }
        if !any {
            return Ok(global);
        }
        LayerMap::from_samples(c, &samples).map_err(err)
    }

    /// The macro albedo (linear) at level-0 global sample (gx, gz) for macro level `level`, `None` without
    /// [`Self::with_means`].
    pub fn albedo(&self, gx: i64, gz: i64, level: u8, stamps: Option<&StampSet>, cands: &[usize]) -> Option<[f32; 3]> {
        let (global, by_stamp) = self.macros.as_ref()?;
        let pick = stamps.and_then(|set| pick_stamp(&set.stamps, cands, sample_meters(gx), -sample_meters(gz)));
        let p = pick.and_then(|p| by_stamp.get(cands[p]).and_then(Option::as_ref)).unwrap_or(global);
        Some(p.albedo(gx, gz, level))
    }

    /// The stamps [`Self::albedo`] must test inside tile `c` (empty without stamps).
    pub fn tile_candidates(c: TileCoord, stamps: Option<&StampSet>) -> Vec<usize> {
        stamps.map_or_else(Vec::new, |set| Self::candidates(set, Self::tile_rect(c)))
    }

    /// Length of [`Self::gpu_words`] (0 without [`Self::with_means`]).
    pub fn gpu_word_count(&self) -> usize {
        self.macros.as_ref().map_or(0, |(g, s)| 3 + s.len() + g.word_count() + s.iter().flatten().map(Pyramid::word_count).sum::<usize>())
    }

    /// The macro pyramids as GPU field words from word `base` (`tiles.wgsl` `wg_layer_albedo`): a 3-word header
    /// (stamp count, absolute offset of the per-stamp table, absolute offset of the global record), the per-stamp
    /// table (absolute record offset, 0 = no grid), then the records ([`Pyramid::gpu_words`]). Empty without
    /// [`Self::with_means`].
    pub fn gpu_words(&self, base: usize) -> Vec<u32> {
        let Some((global, by_stamp)) = &self.macros else { return Vec::new() };
        let w32 = |v: usize| u32::try_from(v).expect("layer field offset fits u32");
        let table = base + 3;
        let mut at = table + by_stamp.len();
        let mut d = vec![w32(by_stamp.len()), w32(table), w32(at)];
        let mut records = global.gpu_words(at);
        at += records.len();
        for p in by_stamp {
            match p {
                Some(p) => {
                    d.push(w32(at));
                    let r = p.gpu_words(at);
                    at += r.len();
                    records.extend(r);
                }
                None => d.push(0),
            }
        }
        d.extend(records);
        debug_assert_eq!(d.len(), self.gpu_word_count());
        d
    }
}

/// Of `cands` (indices into `stamps`), the position of the stamp of largest weight over
/// [`STAMP_LAYER_WEIGHT`] at guide point (x, y), the first on a tie (as `tiles.wgsl` `wg_stamp_pick`).
fn pick_stamp(stamps: &[Stamp], cands: &[usize], x: f64, y: f64) -> Option<usize> {
    let mut best: Option<(usize, f32)> = None;
    for (p, &k) in cands.iter().enumerate() {
        if let Some((_, w)) = stamps.get(k).and_then(|s| s.sample(x, y))
            && w > STAMP_LAYER_WEIGHT
            && best.is_none_or(|b| w > b.1)
        {
            best = Some((p, w));
        }
    }
    best.map(|b| b.0)
}
