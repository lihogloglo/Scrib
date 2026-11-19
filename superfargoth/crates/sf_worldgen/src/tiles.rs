//! **Runtime terrain from a guide** (worldgen.md "Pipeline" stage 7, slice G1): canonical `sf_terrain` tiles
//! generated on demand, so a game shows a guide at its final resolution (level 0 = 0.5 m) without baking it
//! (a 50 km island at 0.5 m would be ~58 GB). [`GuideTiles`] implements [`TerrainTiles`]: the runtime calls it
//! from its async tile loads, and GPU clipmaps and CPU height queries read the same tiles, so physics and
//! rendering see bitwise the same surface.
//!
//! **Sample.** Tile sample (i, j) of level `L` sits at the absolute engine position of its level-0 global
//! indices (`sf_terrain::sample_meters`). Its height is the guide's Catmull-Rom surface plus
//! [`filter::detail_offset_banded`] there, with the level's band limit. A pure function of the global indices:
//! the same tile on any thread, and neighbours on their shared edge, agree bit for bit.
//!
//! **Frames.** Guide: x east, y north, metres, row 0 = north (`guide.rs`). Engine (`sf_coords`): Y up, X east,
//! −Z north, so engine (X, Z) = guide (x, −y) and the guide's sample (col, row) sits at
//! `X = origin.x + col·s`, `Z = row·s − origin.y`. The guide's metres are the engine's: no offset.
//!
//! **Band limit per level.** Octave `k` of the filter (wavelength `λₖ`) is scaled by
//! `smoothstep(lo · s, hi · s, λₖ)` at sample spacing `s` ([`TileSettings::band_samples`]): an octave is off
//! once it is under `lo` samples per wavelength (Nyquist: 2) and full from `hi`, so coarse levels don't alias
//! and CDLOD morphs between consistent levels. Level 0 at the defaults keeps every octave (the full filter).
//!
//! **Bounds.** A tile's block bounds must cover every finer level beneath it. Every level's surface is the
//! Catmull-Rom of the guide plus at most `A` of detail, so a block's bound is the guide's min/max over the
//! block's Catmull-Rom footprint, widened by the kernel's largest overshoot (`CR_OVERSHOOT` of the range) and by
//! the largest detail amplitude there (`max_amplitude_m`, and `shore_ratio` × height above sea).
//!
//! **Look** (slice WG-X3, `crate::layers`): a guide with `layers.json` gives every tile its layer map
//! ([`TerrainTiles::layer_map`], any level; the runtime asks for its near layer levels) and, once the app hands
//! in the layer arrays and their table ([`GuideTiles::with_layer_arrays`]), the macro albedo from the layers' mean
//! albedos, on the CPU and in `tiles.wgsl` alike. Without layers: a macro albedo by height and slope
//! (`FALLBACK_LOOK`, a named scaffold kept only for guides without layers), which `sf_terrain_bevy` also shows
//! near the camera through its default layer.
//!
//! **GPU twin** (slice G2): [`GuideTiles::gpu_field`] hands the runtime `tiles.wgsl` ([`wgsl`]) and the guide
//! with the parameters ([`GuideTiles::gpu_data`]), so it fills the clipmaps on the GPU; the CPU tiles above stay
//! the reference for height queries, generated on a small pool of their own ([`TileSettings::twin_threads`])
//! so they never compete with the game for every core. CDLOD bounds come from [`GuideTiles::bounds_only`]
//! without evaluating the filter. At runtime the filter runs [`TileSettings::runtime_octaves`] octaves (down to
//! ~1 m wavelength) instead of the preview's declared count.
//!
//! **Stamps** (slice WG-X2s, `crate::stamps`): when the guide folder has `stamps.json`, a stamp's height `h_s`
//! and weight `w` replace the sample: `mix(h_base + detail · (1 − w), h_s, w)`, the detail offset scaled by
//! `1 − w` (so it is zero where `w` = 1, where the stamp's height is returned bit for bit) and samples outside
//! every stamp (or at `w` = 0) bitwise unchanged. The bounds widen by the stamps' heights; the GPU twin packs
//! [`crate::stamps::StampIndex::gpu_words`] after the guide's heights ([`slot::STAMPS`]).

use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Instant;

use rayon::prelude::*;
use sf_terrain::{
    BLOCK_QUADS, BLOCKS, BlockBounds, EncodedHeights, GpuField, HeightTile, LAYER_SAMPLES, LayerArrays, LayerCap,
    LayerMap, LayerTable, MacroProduct, TILE_SAMPLES, TerrainTiles, TileCoord, TileProduct, sample_meters,
};
use sf_world::WorldspaceId;

use crate::filter::{self, BaseField, Declared, FilterParams};
use crate::guide::Guide;
use crate::layers::{GuideLayers, LayerField};
use crate::stamps::{StampIndex, StampSet};
use crate::{Error, Result};

#[cfg(test)]
#[path = "tests_tiles.rs"]
mod tests;

/// Largest Catmull-Rom overshoot past the range of its 4 × 4 taps, as a share of that range: the positive
/// weights of the tensor kernel sum to at most (Σ|w| + 1) / 2 = (1.25² + 1) / 2.
// tunables: const (maths: bound of the Catmull-Rom kernel, max Σ|w| = 1.25 per axis at t = 0.5)
const CR_OVERSHOOT: f64 = 0.281_25;

/// Levels the timing counters keep (canonical tiles have 10 height levels).
// tunables: const (layout: counter slots, one per tile level)
const STAT_LEVELS: usize = 16;

/// The declared tile values (spec `tunables.md` "Definition").
pub fn declared_tile_defaults() -> Vec<Declared> {
    vec![
        Declared {
            name: "band_lo_samples",
            v: 2.0,
            unit: "samples",
            min: 1.0,
            max: 16.0,
            source: "Industry(Nyquist: a wavelength needs at least 2 samples)".into(),
            help: "filter octaves under this many samples per wavelength at a level's spacing are off there",
        },
        Declared {
            name: "band_hi_samples",
            v: 4.0,
            unit: "samples",
            min: 1.0,
            max: 32.0,
            source: "Guess".into(),
            help: "filter octaves of at least this many samples per wavelength are at full strength (smooth fade between)",
        },
        Declared {
            name: "runtime_octaves",
            v: 8.0,
            unit: "count",
            min: 0.0,
            max: 8.0,
            source: "Guess".into(),
            help: "filter octaves at runtime: 8 from 160 m at lacunarity 2 end at 1.25 m, the finest a 0.5 m level 0 \
                   carries (band_lo 2 samples = 1 m); the preview's declared octaves stay for 5 m crops",
        },
        Declared {
            name: "twin_threads",
            v: 2.0,
            unit: "count",
            min: 1.0,
            max: 64.0,
            source: "Guess".into(),
            help: "threads of the CPU twin's own pool (height-query tiles), so it never takes every core from the game",
        },
    ]
}

/// Tile generation settings, from [`declared_tile_defaults`].
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TileSettings {
    /// Band limit `[lo, hi]` in samples per wavelength.
    pub band_samples: [f32; 2],
    /// Filter octaves at runtime ([`GuideTiles::open`] sets the filter's `octaves` to it).
    pub runtime_octaves: u32,
    /// Threads of the CPU twin's pool ([`GuideTiles::with_twin_pool`]).
    pub twin_threads: usize,
}

impl Default for TileSettings {
    fn default() -> Self {
        Self::from_declared(&declared_tile_defaults()).expect("defaults are in range")
    }
}

impl TileSettings {
    pub fn from_declared(d: &[Declared]) -> Result<Self> {
        let get = |name: &str| -> Result<f32> {
            let p = d
                .iter()
                .find(|p| p.name == name)
                .ok_or_else(|| Error::Invalid(format!("tile value {name} not declared")))?;
            if p.v.is_finite() && p.v >= p.min && p.v <= p.max {
                Ok(p.v as f32)
            } else {
                Err(Error::Invalid(format!("{name} = {} outside [{}, {}]", p.v, p.min, p.max)))
            }
        };
        let band_samples = [get("band_lo_samples")?, get("band_hi_samples")?];
        if band_samples[0] >= band_samples[1] {
            return Err(Error::Invalid("band_lo_samples < band_hi_samples required".into()));
        }
        let whole = |name: &str| -> Result<u32> {
            let v = get(name)?;
            if v.fract() == 0.0 { Ok(v as u32) } else { Err(Error::Invalid(format!("{name} = {v} must be an integer"))) }
        };
        Ok(Self { band_samples, runtime_octaves: whole("runtime_octaves")?, twin_threads: whole("twin_threads")? as usize })
    }
}

/// Tile generation time of one level, since the provider was made.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct LevelTiming {
    pub level: u8,
    /// Height tiles generated (heights + bounds).
    pub tiles: u64,
    pub total_ms: f64,
    pub max_ms: f64,
    /// Macro albedo tiles and their total time.
    pub macros: u64,
    pub macro_ms: f64,
}

#[derive(Default)]
struct LevelCounters {
    tiles: AtomicU64,
    total_us: AtomicU64,
    max_us: AtomicU64,
    macros: AtomicU64,
    macro_us: AtomicU64,
}

/// Min/max of 2^k × 2^k blocks of the guide (k ≥ 1), for conservative block bounds.
struct MinMax {
    w: usize,
    min: Vec<f32>,
    max: Vec<f32>,
}

/// Where a generated world's layer arrays come from (`TerrainTiles::layers_capped`): the arrays of a bake that
/// has the guide's layer records.
pub type LayerArraysFn =
    Arc<dyn Fn(Option<u32>, &LayerCap) -> Option<std::result::Result<LayerArrays, String>> + Send + Sync>;

/// A guide as runtime terrain tiles ([`TerrainTiles`]).
pub struct GuideTiles {
    ws: WorldspaceId,
    guide: Arc<Guide>,
    prm: FilterParams,
    settings: TileSettings,
    /// Levels 1.. of the min/max pyramid (index 0 = 2 × 2 blocks).
    pyramid: Arc<Vec<MinMax>>,
    fill: f32,
    stats: [LevelCounters; STAT_LEVELS],
    /// Where tile heights are generated: this pool, or rayon's global one.
    pool: Option<Arc<rayon::ThreadPool>>,
    /// The guide's height stamps, indexed.
    stamps: Option<Arc<StampIndex>>,
    /// The guide's ground layers (`crate::layers`).
    layers: Option<Arc<LayerField>>,
    /// The layer arrays the layers index.
    arrays: Option<LayerArraysFn>,
}

impl GuideTiles {
    /// Reads a guide directory (v1) with the declared filter and tile defaults, the filter at
    /// [`TileSettings::runtime_octaves`].
    pub fn open(dir: &Path, ws: WorldspaceId) -> Result<Self> {
        let guide = Guide::read(dir)?;
        let settings = TileSettings::default();
        let prm = FilterParams { octaves: settings.runtime_octaves, ..FilterParams::default() };
        let mut tiles = Self::new(guide, ws, prm, settings);
        if let Some(set) = StampSet::read(dir)? {
            tiles = tiles.with_stamps(set);
        }
        match GuideLayers::read(dir)? {
            Some(layers) => tiles.with_layers(layers),
            None => Ok(tiles),
        }
    }

    pub fn new(guide: Guide, ws: WorldspaceId, prm: FilterParams, settings: TileSettings) -> Self {
        let pyramid = Arc::new(build_pyramid(&guide));
        let (w, h) = (guide.w, guide.h);
        let edge = (0..w)
            .flat_map(|c| [guide.heights[c], guide.heights[(h - 1) * w + c]])
            .chain((0..h).flat_map(|r| [guide.heights[r * w], guide.heights[r * w + w - 1]]));
        let fill = edge.fold(f32::INFINITY, f32::min);
        Self {
            ws,
            guide: Arc::new(guide),
            prm,
            settings,
            pyramid,
            fill,
            stats: Default::default(),
            pool: None,
            stamps: None,
            layers: None,
            arrays: None,
        }
    }

    /// These tiles with height stamps (`crate::stamps`; an empty set means none).
    #[must_use]
    pub fn with_stamps(mut self, set: StampSet) -> Self {
        self.stamps = (!set.stamps.is_empty()).then(|| Arc::new(StampIndex::new(set)));
        self
    }

    /// The height stamps, when the guide has any.
    pub fn stamps(&self) -> Option<&StampIndex> {
        self.stamps.as_deref()
    }

    /// These tiles with ground layers (`crate::layers`; stamp grids index the stamps, so set those first).
    pub fn with_layers(mut self, layers: GuideLayers) -> Result<Self> {
        let n = self.stamps.as_ref().map_or(0, |st| st.set().stamps.len());
        self.layers = Some(Arc::new(LayerField::new(layers, n)?));
        Ok(self)
    }

    /// The ground layers, when the guide has any.
    pub fn layers(&self) -> Option<&LayerField> {
        self.layers.as_deref()
    }

    /// These tiles drawing the layer arrays `arrays` (whose table is `table`): the macro look then comes from the
    /// layers' mean albedos. Returns the palette records `table` doesn't have (drawn neutral grey). Without
    /// ground layers only the arrays are kept (tiles name no layer: the table's fill layer everywhere).
    pub fn with_layer_arrays(mut self, table: &LayerTable, arrays: LayerArraysFn) -> Result<(Self, Vec<String>)> {
        let mut missing = Vec::new();
        if let Some(field) = &self.layers {
            let n = self.stamps.as_ref().map_or(0, |st| st.set().stamps.len());
            let mut f = LayerField::new(field.layers().clone(), n)?;
            missing = f.with_means(table);
            self.layers = Some(Arc::new(f));
        }
        self.arrays = Some(arrays);
        Ok((self, missing))
    }

    /// The same guide (shared, nothing copied) with other filter parameters: a live filter change. The
    /// timing counters start over; the twin pool is shared.
    #[must_use]
    pub fn with_params(&self, prm: FilterParams) -> Self {
        Self {
            ws: self.ws.clone(),
            guide: self.guide.clone(),
            prm,
            settings: self.settings,
            pyramid: self.pyramid.clone(),
            fill: self.fill,
            stats: Default::default(),
            pool: self.pool.clone(),
            stamps: self.stamps.clone(),
            layers: self.layers.clone(),
            arrays: self.arrays.clone(),
        }
    }

    /// Tile heights are generated on a pool of [`TileSettings::twin_threads`] threads of their own (the
    /// CPU twin of a GPU-filled world) instead of rayon's global pool.
    pub fn with_twin_pool(mut self) -> Result<Self> {
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(self.settings.twin_threads)
            .thread_name(|i| format!("terrain-twin-{i}"))
            .build()
            .map_err(|e| Error::Invalid(format!("terrain twin pool: {e}")))?;
        self.pool = Some(Arc::new(pool));
        Ok(self)
    }

    pub fn guide(&self) -> &Guide {
        &self.guide
    }

    pub fn params(&self) -> &FilterParams {
        &self.prm
    }

    pub fn settings(&self) -> &TileSettings {
        &self.settings
    }

    fn base(&self) -> BaseField<'_> {
        BaseField {
            w: self.guide.w,
            h: self.guide.h,
            spacing_m: self.guide.spacing_m,
            origin_m: self.guide.origin_m,
            heights: &self.guide.heights,
        }
    }

    /// The guide's extent in engine metres: (x0, z0, x1, z1).
    pub fn extent_m(&self) -> [f64; 4] {
        let g = &self.guide;
        let s = g.spacing_m;
        [g.origin_m[0], -g.origin_m[1], g.origin_m[0] + (g.w - 1) as f64 * s, (g.h - 1) as f64 * s - g.origin_m[1]]
    }

    /// The guide's highest sample as an engine position (x, height, z).
    pub fn summit(&self) -> [f64; 3] {
        let g = &self.guide;
        let k = (0..g.heights.len()).fold(0, |m, k| if g.heights[k] > g.heights[m] { k } else { m });
        let (c, r) = ((k % g.w) as f64, (k / g.w) as f64);
        [g.origin_m[0] + c * g.spacing_m, f64::from(g.heights[k]), r * g.spacing_m - g.origin_m[1]]
    }

    /// The band limit `[lo, hi]` in metres of wavelength at `level`.
    pub fn band_m(&self, level: u8) -> [f32; 2] {
        let s = TileCoord::new(level, 0, 0).spacing();
        [self.settings.band_samples[0] * s, self.settings.band_samples[1] * s]
    }

    /// Whether every filter octave is off at `level` (the largest is under the band).
    fn all_octaves_off(&self, level: u8) -> bool {
        self.prm.octaves == 0 || self.prm.wavelength_m <= self.band_m(level)[0]
    }

    /// The height at engine position (x, z) metres, as level `level` sees it (its band limit).
    pub fn height(&self, x: f64, z: f64, level: u8) -> f32 {
        self.height_with(&self.base(), x, z, self.band_m(level), self.all_octaves_off(level))
    }

    fn height_with(&self, base: &BaseField<'_>, x: f64, z: f64, band: [f32; 2], off: bool) -> f32 {
        let g = &self.guide;
        let col = (x - g.origin_m[0]) / g.spacing_m;
        let row = (g.origin_m[1] + z) / g.spacing_m;
        let (hs, w) = self.stamps.as_ref().and_then(|st| st.sample(x, -z)).unwrap_or((0.0, 0.0));
        if w >= 1.0 {
            return hs;
        }
        let h = if off {
            base.height(col, row) as f32
        } else {
            let s = base.sample(col, row);
            let d = filter::detail_offset_banded([x, -z], &s, g.sea_level_m, &self.prm, Some(band));
            // Outside every stamp the sum stays the unstamped one, bitwise.
            if w > 0.0 { s.h as f32 + d * (1.0 - w) } else { s.h as f32 + d }
        };
        if w > 0.0 { h + (hs - h) * w } else { h }
    }

    /// Tile `c`'s heights (rows in parallel; each sample a pure function of its global indices).
    pub fn heights(&self, c: TileCoord) -> HeightTile {
        let base = self.base();
        let band = self.band_m(c.level);
        let off = self.all_octaves_off(c.level);
        let mut heights = vec![0.0f32; TILE_SAMPLES * TILE_SAMPLES];
        let fill = |heights: &mut [f32]| {
            heights.par_chunks_mut(TILE_SAMPLES).enumerate().for_each(|(j, row)| {
                for (i, h) in row.iter_mut().enumerate() {
                    let (gx, gz) = c.global_sample(i, j);
                    *h = self.height_with(&base, sample_meters(gx), sample_meters(gz), band, off);
                }
            });
        };
        match &self.pool {
            Some(pool) => pool.install(|| fill(&mut heights)),
            None => fill(&mut heights),
        }
        HeightTile { coord: c, heights }
    }

    /// Guide min/max over columns `c0..=c1`, rows `r0..=r1` (clamped to the guide; a superset when coarse).
    fn guide_range(&self, c: [isize; 2], r: [isize; 2]) -> (f32, f32) {
        let g = &self.guide;
        let cl = |v: isize, n: usize| v.clamp(0, n as isize - 1) as usize;
        let (c0, c1, r0, r1) = (cl(c[0], g.w), cl(c[1], g.w), cl(r[0], g.h), cl(r[1], g.h));
        let span = (c1 - c0 + 1).max(r1 - r0 + 1);
        let mut k = 0;
        while (span >> k) > 8 && k < self.pyramid.len() {
            k += 1;
        }
        let (mut lo, mut hi) = (f32::INFINITY, f32::NEG_INFINITY);
        if k == 0 {
            for r in r0..=r1 {
                for &v in &g.heights[r * g.w + c0..=r * g.w + c1] {
                    lo = lo.min(v);
                    hi = hi.max(v);
                }
            }
        } else {
            let m = &self.pyramid[k - 1];
            for r in (r0 >> k)..=(r1 >> k) {
                for c in (c0 >> k)..=(c1 >> k) {
                    lo = lo.min(m.min[r * m.w + c]);
                    hi = hi.max(m.max[r * m.w + c]);
                }
            }
        }
        (lo, hi)
    }

    /// Conservative block bounds of tile `c` and every finer level under it, grown to cover `tile`'s samples.
    pub fn bounds(&self, c: TileCoord, tile: &HeightTile) -> BlockBounds {
        let mut out = BlockBounds::of(tile);
        let analytic = self.bounds_only(c);
        for k in 0..BLOCKS * BLOCKS {
            out.min[k] = out.min[k].min(analytic.min[k]);
            out.max[k] = out.max[k].max(analytic.max[k]);
        }
        out
    }

    /// Conservative block bounds of tile `c` and every finer level under it, from the guide's min/max pyramid
    /// alone (no sample is evaluated): what a GPU-filled world culls with.
    pub fn bounds_only(&self, c: TileCoord) -> BlockBounds {
        let g = &self.guide;
        let sp = f64::from(c.spacing());
        let (gx0, gz0) = c.first_sample();
        let (x0, z0) = (sample_meters(gx0), sample_meters(gz0));
        let block_m = BLOCK_QUADS as f64 * sp;
        let taps = |a: f64, b: f64| [a.floor() as isize - 1, b.floor() as isize + 2];
        let sea = g.sea_level_m as f32;
        let mut out = BlockBounds { min: vec![0.0; BLOCKS * BLOCKS], max: vec![0.0; BLOCKS * BLOCKS] };
        for bz in 0..BLOCKS {
            for bx in 0..BLOCKS {
                let (xa, xb) = (x0 + bx as f64 * block_m, x0 + (bx + 1) as f64 * block_m);
                let (za, zb) = (z0 + bz as f64 * block_m, z0 + (bz + 1) as f64 * block_m);
                let cols = taps((xa - g.origin_m[0]) / g.spacing_m, (xb - g.origin_m[0]) / g.spacing_m);
                let rows = taps((g.origin_m[1] + za) / g.spacing_m, (g.origin_m[1] + zb) / g.spacing_m);
                let (lo, hi) = self.guide_range(cols, rows);
                let over = (CR_OVERSHOOT * f64::from(hi - lo)) as f32;
                let (lo, hi) = (lo - over, hi + over);
                let amp = if self.prm.octaves == 0 {
                    0.0
                } else {
                    self.prm.max_amplitude_m.min(self.prm.shore_ratio * (hi - sea).max(0.0))
                };
                // f32 rounding of the sums: a centimetre plus a few ulps of the height.
                let eps = 0.01 + 1e-6 * lo.abs().max(hi.abs());
                let k = bz * BLOCKS + bx;
                out.min[k] = lo - amp - eps;
                out.max[k] = hi + amp + eps;
                // A stamped sample lies between its generated height (within the above) and its stamp height.
                if let Some((slo, shi)) = self.stamps.as_ref().and_then(|st| st.height_range([xa, -zb, xb, -za])) {
                    let seps = 0.01 + 1e-6 * slo.abs().max(shi.abs());
                    out.min[k] = out.min[k].min(slo - seps);
                    out.max[k] = out.max[k].max(shi + seps);
                }
            }
        }
        out
    }

    /// Macro albedo of tile `c` (sRGB RGBA8 at the layer-map spacing): from the layers' mean albedos when the
    /// guide has layers and their table ([`Self::with_layer_arrays`]), else the fallback look by height and slope
    /// of the guide's surface.
    pub fn macro_rgba(&self, c: TileCoord) -> Vec<[u8; 4]> {
        let mut out = vec![[0u8; 4]; LAYER_SAMPLES * LAYER_SAMPLES];
        if let Some(field) = self.layers.as_deref().filter(|f| f.has_macro()) {
            let set = self.stamps.as_ref().map(|st| st.set());
            let cands = LayerField::tile_candidates(c, set);
            out.par_chunks_mut(LAYER_SAMPLES).enumerate().for_each(|(j, row)| {
                for (i, px) in row.iter_mut().enumerate() {
                    let (gx, gz) = c.global_sample(i * 2, j * 2);
                    let rgb = field.albedo(gx, gz, c.level, set, &cands).expect("macro pyramids");
                    *px = [srgb(rgb[0]), srgb(rgb[1]), srgb(rgb[2]), 255];
                }
            });
            return out;
        }
        let base = self.base();
        let g = &self.guide;
        out.par_chunks_mut(LAYER_SAMPLES).enumerate().for_each(|(j, row)| {
            for (i, px) in row.iter_mut().enumerate() {
                let (gx, gz) = c.global_sample(i * 2, j * 2);
                let (x, z) = (sample_meters(gx), sample_meters(gz));
                let s = base.sample((x - g.origin_m[0]) / g.spacing_m, (g.origin_m[1] + z) / g.spacing_m);
                let slope = (s.grad[0] * s.grad[0] + s.grad[1] * s.grad[1]).sqrt();
                let rgb = fallback_look((s.h - g.sea_level_m) as f32, slope);
                *px = [srgb(rgb[0]), srgb(rgb[1]), srgb(rgb[2]), 255];
            }
        });
        out
    }

    /// Cumulative generation time per level (levels that generated anything).
    pub fn timings(&self) -> Vec<LevelTiming> {
        self.stats
            .iter()
            .enumerate()
            .filter(|(_, s)| s.tiles.load(Ordering::Relaxed) + s.macros.load(Ordering::Relaxed) > 0)
            .map(|(l, s)| LevelTiming {
                level: l as u8,
                tiles: s.tiles.load(Ordering::Relaxed),
                total_ms: s.total_us.load(Ordering::Relaxed) as f64 / 1000.0,
                max_ms: s.max_us.load(Ordering::Relaxed) as f64 / 1000.0,
                macros: s.macros.load(Ordering::Relaxed),
                macro_ms: s.macro_us.load(Ordering::Relaxed) as f64 / 1000.0,
            })
            .collect()
    }

    fn counters(&self, level: u8) -> &LevelCounters {
        &self.stats[usize::from(level).min(STAT_LEVELS - 1)]
    }

    /// The tile product of `c`: heights (f32, no quantisation), bounds, no layers or tint (layer maps come alone,
    /// [`TerrainTiles::layer_map`], so the CPU twin's height tiles never pay for them).
    pub fn product(&self, c: TileCoord) -> TileProduct {
        let t = Instant::now();
        let tile = self.heights(c);
        let bounds = self.bounds(c, &tile);
        let us = t.elapsed().as_micros() as u64;
        let s = self.counters(c.level);
        s.tiles.fetch_add(1, Ordering::Relaxed);
        s.total_us.fetch_add(us, Ordering::Relaxed);
        s.max_us.fetch_max(us, Ordering::Relaxed);
        TileProduct { coord: c, heights: EncodedHeights::F32(tile.heights), bounds, layers: None, tint: None }
    }
}

impl TerrainTiles for GuideTiles {
    fn worldspace(&self) -> &WorldspaceId {
        &self.ws
    }

    /// Every tile exists: past the guide its edge extends (the guide's edges are sea).
    fn has_tile(&self, _: TileCoord) -> bool {
        true
    }

    fn tile(&self, c: TileCoord) -> Option<std::result::Result<TileProduct, String>> {
        Some(Ok(self.product(c)))
    }

    fn macro_albedo(&self, c: TileCoord) -> Option<std::result::Result<MacroProduct, String>> {
        let t = Instant::now();
        let rgba = self.macro_rgba(c);
        let s = self.counters(c.level);
        s.macros.fetch_add(1, Ordering::Relaxed);
        s.macro_us.fetch_add(t.elapsed().as_micros() as u64, Ordering::Relaxed);
        Some(Ok(MacroProduct { coord: c, rgba }))
    }

    /// The app's layer arrays ([`GuideTiles::with_layer_arrays`]), their fill height this guide's.
    fn layers_capped(&self, size: Option<u32>, cap: &LayerCap) -> Option<std::result::Result<LayerArrays, String>> {
        let mut a = (self.arrays.as_ref()?)(size, cap)?;
        if let Ok(a) = &mut a {
            a.table.fill_height = self.fill;
        }
        Some(a)
    }

    fn layer_map(&self, c: TileCoord) -> Option<std::result::Result<LayerMap, String>> {
        let field = self.layers.as_ref()?;
        Some(field.layer_map(c, self.stamps.as_ref().map(|st| st.set())).map_err(|e| e.to_string()))
    }

    fn has_layer_maps(&self) -> bool {
        self.layers.is_some()
    }

    fn fill_height(&self) -> f32 {
        self.fill
    }

    fn gpu_field(&self) -> Option<GpuField> {
        Some(GpuField { wgsl: wgsl().into(), data: self.gpu_data().into(), label: "sf_worldgen guide tiles".into() })
    }

    fn block_bounds(&self, c: TileCoord) -> Option<BlockBounds> {
        Some(self.bounds_only(c))
    }
}

/// Header slots of [`GuideTiles::gpu_data`] (`tiles.wgsl` reads them as the `FH_*` constants [`wgsl`] prepends).
pub mod slot {
    pub const W: u32 = 0;
    pub const H: u32 = 1;
    /// Level-0 samples per guide cell when whole (else 0: the f32 affine map).
    pub const CELL_S: u32 = 2;
    pub const COL_CI: u32 = 3;
    pub const COL_CF: u32 = 4;
    pub const ROW_CI: u32 = 5;
    pub const ROW_CF: u32 = 6;
    pub const COL_A: u32 = 7;
    pub const COL_C: u32 = 8;
    pub const ROW_A: u32 = 9;
    pub const ROW_C: u32 = 10;
    pub const SPACING: u32 = 11;
    pub const SEA: u32 = 12;
    pub const OCTAVES: u32 = 13;
    pub const WAVELENGTH: u32 = 14;
    pub const LACUNARITY: u32 = 15;
    pub const GAIN: u32 = 16;
    pub const RELIEF: u32 = 17;
    pub const MAX_AMP: u32 = 18;
    pub const SLOPE_LO: u32 = 19;
    pub const SLOPE_HI: u32 = 20;
    pub const CURV_LO: u32 = 21;
    pub const CURV_HI: u32 = 22;
    pub const SHARP: u32 = 23;
    pub const CARVE: u32 = 24;
    pub const PHASE_FADE: u32 = 25;
    pub const SHORE_RATIO: u32 = 26;
    pub const CASCADE: u32 = 27;
    pub const SEED: u32 = 28;
    pub const BAND_LO: u32 = 29;
    pub const BAND_HI: u32 = 30;
    /// Word of the stamp section ([`crate::stamps::StampIndex::gpu_words`], after the heights); 0 = no stamps.
    pub const STAMPS: u32 = 31;
    /// The fallback look: 6 albedos (18 floats), then 5 breaks (10 floats).
    pub const LOOK: u32 = 32;
    /// Word of the layer macro section ([`crate::layers::LayerField::gpu_words`], after the stamps); 0 = none
    /// (the fallback look).
    pub const LAYERS: u32 = 60;
    /// The guide's heights, row-major.
    pub const HEIGHTS: u32 = 64;

    pub(super) const ALL: &[(&str, u32)] = &[
        ("W", W),
        ("H", H),
        ("CELL_S", CELL_S),
        ("COL_CI", COL_CI),
        ("COL_CF", COL_CF),
        ("ROW_CI", ROW_CI),
        ("ROW_CF", ROW_CF),
        ("COL_A", COL_A),
        ("COL_C", COL_C),
        ("ROW_A", ROW_A),
        ("ROW_C", ROW_C),
        ("SPACING", SPACING),
        ("SEA", SEA),
        ("OCTAVES", OCTAVES),
        ("WAVELENGTH", WAVELENGTH),
        ("LACUNARITY", LACUNARITY),
        ("GAIN", GAIN),
        ("RELIEF", RELIEF),
        ("MAX_AMP", MAX_AMP),
        ("SLOPE_LO", SLOPE_LO),
        ("SLOPE_HI", SLOPE_HI),
        ("CURV_LO", CURV_LO),
        ("CURV_HI", CURV_HI),
        ("SHARP", SHARP),
        ("CARVE", CARVE),
        ("PHASE_FADE", PHASE_FADE),
        ("SHORE_RATIO", SHORE_RATIO),
        ("CASCADE", CASCADE),
        ("SEED", SEED),
        ("BAND_LO", BAND_LO),
        ("BAND_HI", BAND_HI),
        ("STAMPS", STAMPS),
        ("LOOK", LOOK),
        ("LAYERS", LAYERS),
        ("HEIGHTS", HEIGHTS),
    ];
}

/// `tiles.wgsl` with its header constants: `field_height` and `field_albedo` over a host-declared
/// `field_data: array<u32>` holding [`GuideTiles::gpu_data`].
pub fn wgsl() -> String {
    use std::fmt::Write as _;
    let mut out = String::from("// sf_worldgen tiles.rs: header slots of GuideTiles::gpu_data.\n");
    for (name, v) in slot::ALL {
        let _ = writeln!(out, "const FH_{name}: u32 = {v}u;");
    }
    let _ = writeln!(out, "const FILTER_KERNEL_RADIUS: f32 = {:?};", filter::KERNEL_RADIUS_CELLS);
    out.push_str(include_str!("tiles.wgsl"));
    out
}

impl GuideTiles {
    /// The words `tiles.wgsl` reads (`field_data`): the header ([`slot`]), the guide's heights, then the stamps
    /// when there are any. One storage binding: 128 MiB at wgpu's default limits ([`Self::gpu_data_bytes`]).
    pub fn gpu_data(&self) -> Vec<u32> {
        let g = &self.guide;
        let p = &self.prm;
        let mut d = vec![0u32; slot::HEIGHTS as usize + g.heights.len()];
        // Level-0 sample index n → guide column (n + cc) / cell, row (n + rc) / cell (`tiles.wgsl` wg_split):
        // x = n / 2 − 128, col = (x − origin.x) / spacing; z likewise, row = (origin.y + z) / spacing.
        let cell = 2.0 * g.spacing_m;
        let cc = -256.0 - 2.0 * g.origin_m[0];
        let rc = 2.0 * g.origin_m[1] - 256.0;
        let whole = cell.fract() == 0.0 && (1.0..=1_048_576.0).contains(&cell) && cc.abs() < 1e9 && rc.abs() < 1e9;
        let split = |v: f64| {
            let i = v.floor();
            let frac = (v - i) as f32;
            if frac >= 1.0 { (i as i32 + 1, 0.0) } else { (i as i32, frac) }
        };
        let (col_int, col_frac) = split(cc);
        let (row_int, row_frac) = split(rc);
        let l = &FALLBACK_LOOK;
        let look = [l.sea_floor, l.sand, l.scrub, l.ash, l.high_rock, l.cliff].concat();
        let breaks = [l.wet_m, l.beach_m, l.ash_m, l.high_m, l.cliff_slope].concat();
        let floats = [
            (slot::COL_CF, col_frac),
            (slot::ROW_CF, row_frac),
            (slot::COL_A, (0.5 / g.spacing_m) as f32),
            (slot::COL_C, ((-128.0 - g.origin_m[0]) / g.spacing_m) as f32),
            (slot::ROW_A, (0.5 / g.spacing_m) as f32),
            (slot::ROW_C, ((g.origin_m[1] - 128.0) / g.spacing_m) as f32),
            (slot::SPACING, g.spacing_m as f32),
            (slot::SEA, g.sea_level_m as f32),
            (slot::WAVELENGTH, p.wavelength_m),
            (slot::LACUNARITY, p.lacunarity),
            (slot::GAIN, p.gain),
            (slot::RELIEF, p.relief_fraction),
            (slot::MAX_AMP, p.max_amplitude_m),
            (slot::SLOPE_LO, p.slope_fade[0]),
            (slot::SLOPE_HI, p.slope_fade[1]),
            (slot::CURV_LO, p.curvature_fade[0]),
            (slot::CURV_HI, p.curvature_fade[1]),
            (slot::SHARP, p.crest_sharpness),
            (slot::CARVE, p.carve),
            (slot::PHASE_FADE, p.phase_fade),
            (slot::SHORE_RATIO, p.shore_ratio),
            (slot::BAND_LO, self.settings.band_samples[0]),
            (slot::BAND_HI, self.settings.band_samples[1]),
        ];
        for (s, v) in floats {
            d[s as usize] = v.to_bits();
        }
        for (k, v) in look.iter().chain(&breaks).enumerate() {
            d[slot::LOOK as usize + k] = v.to_bits();
        }
        d[slot::W as usize] = g.w as u32;
        d[slot::H as usize] = g.h as u32;
        d[slot::CELL_S as usize] = if whole { cell as u32 } else { 0 };
        d[slot::COL_CI as usize] = col_int.cast_unsigned();
        d[slot::ROW_CI as usize] = row_int.cast_unsigned();
        d[slot::OCTAVES as usize] = p.octaves;
        d[slot::CASCADE as usize] = u32::from(p.cascade);
        d[slot::SEED as usize] = p.seed;
        for (k, h) in g.heights.iter().enumerate() {
            d[slot::HEIGHTS as usize + k] = h.to_bits();
        }
        if let Some(st) = &self.stamps {
            let at = d.len();
            d[slot::STAMPS as usize] = u32::try_from(at).expect("field words fit u32");
            d.extend(st.gpu_words(at));
        }
        if let Some(field) = self.layers.as_deref().filter(|f| f.has_macro()) {
            let at = d.len();
            d[slot::LAYERS as usize] = u32::try_from(at).expect("field words fit u32");
            d.extend(field.gpu_words(at));
        }
        d
    }

    /// Bytes of [`Self::gpu_data`] (what the GPU field's one storage binding must hold).
    pub fn gpu_data_bytes(&self) -> usize {
        4 * (slot::HEIGHTS as usize
            + self.guide.heights.len()
            + self.stamps.as_ref().map_or(0, |st| st.gpu_word_count())
            + self.layers.as_ref().map_or(0, |f| f.gpu_word_count()))
    }
}

fn build_pyramid(g: &Guide) -> Vec<MinMax> {
    let mut levels: Vec<MinMax> = Vec::new();
    let (mut w, mut h) = (g.w, g.h);
    loop {
        let (nw, nh) = (w.div_ceil(2), h.div_ceil(2));
        let mut min = vec![f32::INFINITY; nw * nh];
        let mut max = vec![f32::NEG_INFINITY; nw * nh];
        for r in 0..h {
            for c in 0..w {
                let k = (r / 2) * nw + c / 2;
                let (lo, hi) = match levels.last() {
                    None => (g.heights[r * w + c], g.heights[r * w + c]),
                    Some(p) => (p.min[r * w + c], p.max[r * w + c]),
                };
                min[k] = min[k].min(lo);
                max[k] = max[k].max(hi);
            }
        }
        levels.push(MinMax { w: nw, min, max });
        if nw == 1 && nh == 1 {
            return levels;
        }
        (w, h) = (nw, nh);
    }
}

/// Linear 0..1 → sRGB 8-bit.
fn srgb(v: f32) -> u8 {
    let v = v.clamp(0.0, 1.0);
    let s = if v <= 0.003_130_8 { v * 12.92 } else { 1.055 * v.powf(1.0 / 2.4) - 0.055 };
    (s * 255.0).round() as u8
}

fn smoothstep(lo: f32, hi: f32, x: f32) -> f32 {
    let t = ((x - lo) / (hi - lo)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

fn mix(a: [f32; 3], b: [f32; 3], t: f32) -> [f32; 3] {
    [a[0] + (b[0] - a[0]) * t, a[1] + (b[1] - a[1]) * t, a[2] + (b[2] - a[2]) * t]
}

/// Scaffold `FALLBACK_LOOK` (removal path: every guide carries ground layers, `crate::layers`; until then it is
/// the look of guides without them): linear albedos and their height (m above sea) and slope (rise over run)
/// breaks.
// tunables: const (scaffold FallbackLook: a placeholder palette until generated ground layers exist)
const FALLBACK_LOOK: FallbackLook = FallbackLook {
    sea_floor: [0.20, 0.18, 0.14],
    sand: [0.38, 0.33, 0.24],
    scrub: [0.10, 0.12, 0.06],
    ash: [0.17, 0.14, 0.11],
    high_rock: [0.09, 0.08, 0.075],
    cliff: [0.16, 0.15, 0.14],
    wet_m: [-10.0, 0.0],
    beach_m: [2.0, 10.0],
    ash_m: [300.0, 1200.0],
    high_m: [2000.0, 3200.0],
    cliff_slope: [0.6, 1.0],
};

struct FallbackLook {
    sea_floor: [f32; 3],
    sand: [f32; 3],
    scrub: [f32; 3],
    ash: [f32; 3],
    high_rock: [f32; 3],
    cliff: [f32; 3],
    wet_m: [f32; 2],
    beach_m: [f32; 2],
    ash_m: [f32; 2],
    high_m: [f32; 2],
    cliff_slope: [f32; 2],
}

/// The fallback albedo (linear) at height `h` above sea (m) and slope (rise over run).
fn fallback_look(h: f32, slope: f32) -> [f32; 3] {
    let l = &FALLBACK_LOOK;
    if h < 0.0 {
        return mix(l.sea_floor, l.sand, smoothstep(l.wet_m[0], l.wet_m[1], h));
    }
    let land = mix(l.scrub, l.ash, smoothstep(l.ash_m[0], l.ash_m[1], h));
    let land = mix(land, l.high_rock, smoothstep(l.high_m[0], l.high_m[1], h));
    let c = mix(l.sand, land, smoothstep(l.beach_m[0], l.beach_m[1], h));
    mix(c, l.cliff, smoothstep(l.cliff_slope[0], l.cliff_slope[1], slope))
}
