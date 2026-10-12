//! Terrain bake steps on `sf_bake` (bake.md "M2 steps", terrain.md).
//!
//! | Step | Subject | Reads | Output |
//! |---|---|---|---|
//! | `terrain.tile` | `ws|0|x|z` | overlapping source tiles (records), footprints (param) | [`TileProduct`] |
//! | `terrain.pyramid` | `ws|k|x|z` | the 4×4 level k−1 tiles around it | [`TileProduct`] |
//! | `terrain.macro` | `ws|k|x|z` | the tile, `terrain.layer_table` | [`MacroProduct`] |
//! | `terrain.layer_table` | `""` | every `terrain.layer` record, albedo files | [`LayerTable`] |
//! | `terrain.layers` | texture size | the table, every layer texture | [`LayerArrays`] |
//! | `terrain.rt_proxy` | `ws|k|x|z` | the tile | [`RtProxy`] |
//!
//! Source tiles are `Input::Record { kind: "terrain.source_tile", id:
//! "<ws>|<source id>" }`; the host's `Inputs` fingerprints them (usually
//! with [`SourceTileInfo::fingerprint`]). Layers are `terrain.layer`
//! records keyed by record id. Footprints come from a [`FootprintSource`]
//! until `world.index` provides them.
//!
//! A source tile change re-runs the level-0 tiles that read it, then only
//! the pyramid tiles whose children's bytes changed (bake.md: unchanged
//! product bytes stop propagation).

use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::sync::{Arc, Mutex, PoisonError};

use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use sf_bake::{BakeError, Baker, PlanCx, ProductKey, RunCx, Step};
use sf_records::RecordId;
use sf_world::WorldspaceId;

use crate::constrain::{FOOTPRINT_FEATHER, Footprint, FootprintMask, constrain};
use crate::layers::{
    LayerId, LayerMap, LayerParams, TintMap, downsample_layers, downsample_tint, resample_layers, resample_tint,
};
use crate::product::{LayerArrays, LayerInfo, LayerTable, MacroProduct, RtProxy, TileProduct, layer_id};
use crate::pyramid::{BlockBounds, EncodedHeights, downsample, top_level};
use crate::source::{SourceMosaic, SourceTile, SourceTileInfo, TerrainSource};
use crate::tile::{BLOCKS, LAYER_SAMPLES, TILE_QUADS, TINT_SAMPLES, TileCoord, global_coords};
use crate::upsample;

/// Record kind of source tile inputs.
pub const SOURCE_TILE_KIND: &str = "terrain.source_tile";
/// Record kind of terrain layers.
pub const LAYER_KIND: &str = "terrain.layer";

pub const TILE_STEP: &str = "terrain.tile";
pub const PYRAMID_STEP: &str = "terrain.pyramid";
pub const MACRO_STEP: &str = "terrain.macro";
pub const LAYER_TABLE_STEP: &str = "terrain.layer_table";
pub const LAYERS_STEP: &str = "terrain.layers";
pub const RT_PROXY_STEP: &str = "terrain.rt_proxy";

/// Record id of a source tile input: `"<ws>|<id>"`.
pub fn source_tile_record(ws: &WorldspaceId, id: crate::SourceTileId) -> String {
    format!("{}|{}", ws.as_str(), id.0)
}

/// Subject of a per-tile product: `"<ws>|<level>|<x>|<z>"`.
pub fn tile_subject(ws: &WorldspaceId, c: TileCoord) -> String {
    format!("{}|{}|{}|{}", ws.as_str(), c.level, c.x, c.z)
}

/// Inverse of [`tile_subject`].
pub fn parse_tile_subject(s: &str) -> Option<(WorldspaceId, TileCoord)> {
    let mut it = s.rsplitn(4, '|');
    let z = it.next()?.parse().ok()?;
    let x = it.next()?.parse().ok()?;
    let level = it.next()?.parse().ok()?;
    let ws = it.next()?;
    Some((WorldspaceId::new(ws), TileCoord::new(level, x, z)))
}

/// The step that makes tile `c`: `terrain.tile` at level 0, else `terrain.pyramid`.
pub fn tile_step(c: TileCoord) -> &'static str {
    if c.level == 0 { TILE_STEP } else { PYRAMID_STEP }
}

/// Grounded placement footprints (terrain.md `constrain`). Until the world
/// index provides them, hosts inject this.
pub trait FootprintSource: Send + Sync {
    /// Footprints that reach level-0 tile `coord` (feather included).
    fn footprints(&self, ws: &WorldspaceId, coord: TileCoord, out: &mut Vec<Footprint>);
    /// Changes whenever [`FootprintSource::footprints`] would return
    /// something else for this tile.
    fn fingerprint(&self, ws: &WorldspaceId, coord: TileCoord) -> [u8; 32];
}

/// Footprints per level-0 tile, with their fingerprint.
type FootprintTiles = HashMap<TileCoord, (Vec<Footprint>, [u8; 32])>;

/// Footprints of `GROUNDED` placements, bucketed by the level-0 tiles they
/// reach: the generic [`FootprintSource`] until `world.index` feeds the
/// bake. `bounds` gives a placement's model-space AABB (min, max), e.g.
/// from `model.info`; placements without bounds are skipped.
#[derive(Default)]
pub struct PlacementFootprints {
    tiles: HashMap<WorldspaceId, FootprintTiles>,
}

impl PlacementFootprints {
    pub fn new() -> Self {
        Self::default()
    }

    /// The footprint of one placement: its bounds' XZ rectangle, scaled,
    /// rotated by the placement's yaw (tilt is ignored) and centered on the
    /// bounds' center.
    pub fn footprint(p: &sf_world::Placement, (min, max): (glam::Vec3, glam::Vec3)) -> Footprint {
        let t = &p.transform;
        let half = (max - min) * 0.5 * t.scale.abs();
        let center = t.rotation * ((min + max) * 0.5 * t.scale);
        let x = t.rotation * glam::Vec3::X;
        Footprint {
            center: t.pos.offset(glam::Vec3::new(center.x, 0.0, center.z)),
            half_extents: glam::Vec2::new(half.x, half.z),
            yaw: (-x.z).atan2(x.x),
        }
    }

    /// Adds every `GROUNDED` placement of `ws` (replacing what `ws` had).
    pub fn set_worldspace<'a>(
        &mut self,
        ws: &WorldspaceId,
        placements: impl IntoIterator<Item = &'a sf_world::Placement>,
        bounds: &dyn Fn(&sf_world::Placement) -> Option<(glam::Vec3, glam::Vec3)>,
        feather: f32,
    ) {
        let mut tiles: HashMap<TileCoord, Vec<Footprint>> = HashMap::new();
        let per = f64::from(u32::try_from(TILE_QUADS).expect("small"));
        for p in placements {
            if !p.flags.contains(sf_world::PlacementFlags::GROUNDED) {
                continue;
            }
            let Some(b) = bounds(p) else { continue };
            let fp = Self::footprint(p, b);
            let reach = f64::from(fp.half_extents.length() + feather) * 2.0;
            let g = global_coords(fp.center);
            #[allow(clippy::cast_possible_truncation)]
            let t = |v: f64| i32::try_from((v / per).floor() as i64).unwrap_or(0);
            for tz in t(g.y - reach)..=t(g.y + reach) {
                for tx in t(g.x - reach)..=t(g.x + reach) {
                    tiles.entry(TileCoord::new(0, tx, tz)).or_default().push(fp);
                }
            }
        }
        let tiles = tiles
            .into_iter()
            .map(|(c, mut fps)| {
                // Stable order whatever the input order, so fingerprints are too.
                let key = |f: &Footprint| {
                    let g = global_coords(f.center);
                    (
                        g.x.to_bits(),
                        g.y.to_bits(),
                        f.half_extents.x.to_bits(),
                        f.half_extents.y.to_bits(),
                        f.yaw.to_bits(),
                    )
                };
                fps.sort_by_key(key);
                let mut bytes = Vec::with_capacity(fps.len() * 28);
                for f in &fps {
                    let (a, b, c, d, e) = key(f);
                    for v in [a, b] {
                        bytes.extend_from_slice(&v.to_le_bytes());
                    }
                    for v in [c, d, e] {
                        bytes.extend_from_slice(&v.to_le_bytes());
                    }
                }
                (c, (fps, sf_bake::Digest::of(&bytes).0))
            })
            .collect();
        self.tiles.insert(ws.clone(), tiles);
    }
}

impl FootprintSource for PlacementFootprints {
    fn footprints(&self, ws: &WorldspaceId, coord: TileCoord, out: &mut Vec<Footprint>) {
        if let Some((fps, _)) = self.tiles.get(ws).and_then(|t| t.get(&coord)) {
            out.extend_from_slice(fps);
        }
    }
    fn fingerprint(&self, ws: &WorldspaceId, coord: TileCoord) -> [u8; 32] {
        self.tiles.get(ws).and_then(|t| t.get(&coord)).map_or([0; 32], |(_, d)| *d)
    }
}

/// No grounded placements.
pub struct NoFootprints;

impl FootprintSource for NoFootprints {
    fn footprints(&self, _: &WorldspaceId, _: TileCoord, _: &mut Vec<Footprint>) {}
    fn fingerprint(&self, _: &WorldspaceId, _: TileCoord) -> [u8; 32] {
        [0; 32]
    }
}

/// A terrain layer (terrain.md `TerrainLayer`). Adapters map their texture
/// records to it; paths are VFS path strings read through the host's
/// `Inputs`. The bake reads it through [`LayerSource`] and fingerprints it
/// as the record `(terrain.layer, id)`, so it needs no serde form here.
#[derive(Clone, Debug, PartialEq)]
pub struct TerrainLayer {
    /// sRGB albedo with height in alpha (alpha 255 everywhere = flat).
    pub albedo_height: String,
    /// Tangent-space normal map, +Y up.
    pub normal: Option<String>,
    /// Ambient occlusion, roughness, metallic.
    pub arm: Option<String>,
    pub tiling_m: f32,
    pub triplanar: bool,
    /// Detail displacement amplitude in meters (≤ 0.08 by default).
    pub displacement_m: f32,
    /// Surface type record (footsteps, deformation material, grass rule).
    pub surface: Option<RecordId>,
}

/// A decoded RGBA8 image.
#[derive(Clone, Debug, PartialEq)]
pub struct LayerImage {
    pub width: u32,
    pub height: u32,
    pub rgba: Vec<u8>,
}

/// The host's terrain layers: records and a texture decoder (adapters
/// know their formats).
pub trait LayerSource: Send + Sync {
    /// Every `TerrainLayer` record id, sorted.
    fn layers(&self) -> Vec<RecordId>;
    fn layer(&self, id: &RecordId) -> Option<TerrainLayer>;
    fn decode(&self, path: &str, bytes: &[u8]) -> Result<LayerImage, String>;
}

/// Settings that change the output (they are step parameters).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TerrainBakeConfig {
    /// Height where no source tile has samples.
    pub fill_height: f32,
    /// Layer blend radius in meters; `None` = the source's layer texel
    /// spacing (bilinear between texels).
    pub blend_radius: Option<f32>,
    /// Layer where no source texel names one.
    pub default_layer: String,
    /// How far around a tile source tiles are read (filter support).
    pub read_margin: f32,
    /// The pyramid goes at least this high (the renderer's coarsest clipmap level).
    pub min_top_level: u8,
    pub feather: f32,
    /// Ray-tracing proxy: every n-th sample.
    pub rt_stride: u32,
}

impl Default for TerrainBakeConfig {
    fn default() -> Self {
        Self {
            fill_height: 0.0,
            blend_radius: None,
            default_layer: String::new(),
            read_margin: 32.0,
            min_top_level: 9,
            feather: FOOTPRINT_FEATHER,
            rt_stride: 4,
        }
    }
}

/// What exists in one worldspace: source tile bounds and the canonical
/// tiles they produce, at every level.
pub struct TerrainPlan {
    pub sources: Vec<SourceTileInfo>,
    /// Every canonical tile, all levels.
    pub tiles: BTreeSet<TileCoord>,
    pub top: u8,
    /// Level-0 tile -> indices into `sources` it reads.
    reads: HashMap<TileCoord, Vec<usize>>,
}

impl TerrainPlan {
    pub fn new(ws: &WorldspaceId, source: &dyn TerrainSource, config: &TerrainBakeConfig) -> Self {
        let mut sources = Vec::new();
        source.tiles(ws, &mut |info| sources.push(info));
        sources.sort_by_key(|i| i.id);
        let per = f64::from(u32::try_from(TILE_QUADS).expect("small"));
        let margin = f64::from(config.read_margin) * 2.0;
        let mut reads: HashMap<TileCoord, Vec<usize>> = HashMap::new();
        let mut level0 = BTreeSet::new();
        #[allow(clippy::cast_possible_truncation)]
        let index = |v: f64| i32::try_from(v.floor() as i64).unwrap_or(if v < 0.0 { i32::MIN } else { i32::MAX });
        for (n, info) in sources.iter().enumerate() {
            let (lo, hi) = (global_coords(info.min), global_coords(info.max));
            // Tiles the source covers (a tile touched only on its border isn't created).
            for tz in index(lo.y / per)..=index(hi.y / per - 1e-9) {
                for tx in index(lo.x / per)..=index(hi.x / per - 1e-9) {
                    level0.insert(TileCoord::new(0, tx, tz));
                }
            }
            // Tiles whose filter support reaches the source.
            for tz in index((lo.y - margin) / per)..=index((hi.y + margin) / per) {
                for tx in index((lo.x - margin) / per)..=index((hi.x + margin) / per) {
                    reads.entry(TileCoord::new(0, tx, tz)).or_default().push(n);
                }
            }
        }
        let top = if level0.is_empty() { 0 } else { top_level(level0.iter().copied()).max(config.min_top_level) };
        let mut tiles = level0.clone();
        let mut level = level0;
        for _ in 0..top {
            level = level.iter().map(|c| c.parent()).collect();
            tiles.extend(level.iter().copied());
        }
        Self { sources, tiles, top, reads }
    }

    /// Source tiles a level-0 tile reads, sorted by id.
    pub fn sources_of(&self, coord: TileCoord) -> impl Iterator<Item = &SourceTileInfo> {
        self.reads.get(&coord).into_iter().flatten().map(|&n| &self.sources[n])
    }
}

/// Shared state of the terrain steps: the adapter's sources and the plans.
pub struct TerrainBake {
    pub source: Arc<dyn TerrainSource>,
    pub footprints: Arc<dyn FootprintSource>,
    pub layers: Arc<dyn LayerSource>,
    pub config: TerrainBakeConfig,
    plans: Mutex<HashMap<WorldspaceId, Arc<TerrainPlan>>>,
}

impl TerrainBake {
    pub fn new(
        source: Arc<dyn TerrainSource>,
        footprints: Arc<dyn FootprintSource>,
        layers: Arc<dyn LayerSource>,
        config: TerrainBakeConfig,
    ) -> Arc<Self> {
        Arc::new(Self { source, footprints, layers, config, plans: Mutex::new(HashMap::new()) })
    }

    /// The worldspace's plan, computed once per session.
    pub fn plan(&self, ws: &WorldspaceId) -> Arc<TerrainPlan> {
        let mut plans = self.plans.lock().unwrap_or_else(PoisonError::into_inner);
        plans.entry(ws.clone()).or_insert_with(|| Arc::new(TerrainPlan::new(ws, &*self.source, &self.config))).clone()
    }

    /// Registers every terrain step.
    pub fn register(self: &Arc<Self>, baker: &mut Baker) {
        baker
            .register(TileStep(self.clone()))
            .register(PyramidStep(self.clone()))
            .register(MacroStep(self.clone()))
            .register(LayerTableStep(self.clone()))
            .register(LayersStep(self.clone()))
            .register(RtProxyStep(self.clone()));
    }

    /// Every terrain product of `ws`, plus the layer arrays at `array_size`.
    pub fn products(&self, ws: &WorldspaceId, array_size: u32) -> Vec<ProductKey> {
        let plan = self.plan(ws);
        let mut out = vec![ProductKey::new(LAYER_TABLE_STEP, ""), ProductKey::new(LAYERS_STEP, array_size.to_string())];
        for &c in &plan.tiles {
            let s = tile_subject(ws, c);
            out.push(ProductKey::new(tile_step(c), s.clone()));
            out.push(ProductKey::new(MACRO_STEP, s.clone()));
            out.push(ProductKey::new(RT_PROXY_STEP, s));
        }
        out
    }

    /// The same products as [`TerrainBake::products`], in dependency
    /// stages: layer table, layer arrays, level-0 tiles, each pyramid
    /// level, then macro and RT proxies. Bake the stages in order
    /// ([`TerrainBake::bake`]).
    ///
    /// Why: `sf_bake` resolves a product's planned inputs with a nested
    /// rayon `par_iter` inside the product's `OnceLock` initializer. With a
    /// deep graph of shared inputs (each pyramid tile reads 16 children,
    /// each child is read by up to 9 parents), a worker can steal a job
    /// that waits on a slot it is itself initializing further up its stack,
    /// and the bake deadlocks. Stages make every input already resolved
    /// when a product plans, so nothing waits. Remove once `sf_bake`
    /// resolves inputs without blocking inside the initializer.
    pub fn product_stages(&self, ws: &WorldspaceId, array_size: u32) -> Vec<Vec<ProductKey>> {
        let plan = self.plan(ws);
        let mut stages = vec![
            vec![ProductKey::new(LAYER_TABLE_STEP, "")],
            vec![ProductKey::new(LAYERS_STEP, array_size.to_string())],
        ];
        for level in 0..=plan.top {
            stages.push(
                plan.tiles
                    .iter()
                    .filter(|c| c.level == level)
                    .map(|&c| ProductKey::new(tile_step(c), tile_subject(ws, c)))
                    .collect(),
            );
        }
        stages.push(
            plan.tiles
                .iter()
                .flat_map(|&c| {
                    let s = tile_subject(ws, c);
                    [ProductKey::new(MACRO_STEP, s.clone()), ProductKey::new(RT_PROXY_STEP, s)]
                })
                .collect(),
        );
        stages
    }

    /// Bakes every product of `ws` stage by stage; returns the failures.
    pub fn bake(&self, baker: &Baker, ws: &WorldspaceId, array_size: u32) -> Vec<(ProductKey, Arc<BakeError>)> {
        let mut failed = Vec::new();
        for stage in self.product_stages(ws, array_size) {
            let report = baker.bake(&stage);
            failed.extend(report.failures().map(|(k, e)| (k.clone(), e.clone())));
        }
        failed
    }

    fn config_bytes(&self) -> Vec<u8> {
        postcard::to_stdvec(&self.config).expect("serializable")
    }

    fn layer_params(&self, src: &SourceMosaic) -> LayerParams {
        #[allow(clippy::cast_possible_truncation)]
        let texel = src.layers.as_ref().map_or(1.0, |l| l.lattice.spacing as f32);
        LayerParams {
            blend_radius: self.config.blend_radius.unwrap_or(texel),
            default_layer: layer_id(&self.config.default_layer),
        }
    }
}

fn subject_of(step: &'static str, subject: &str) -> Result<(WorldspaceId, TileCoord), BakeError> {
    parse_tile_subject(subject).ok_or_else(|| BakeError::step(&ProductKey::new(step, subject), "bad subject"))
}

fn err(step: &'static str, subject: &str, e: impl std::fmt::Display) -> BakeError {
    BakeError::step(&ProductKey::new(step, subject), e)
}

fn read_tile(cx: &mut RunCx<'_>, ws: &WorldspaceId, c: TileCoord) -> Result<TileProduct, BakeError> {
    let subject = tile_subject(ws, c);
    let bytes = cx.product(tile_step(c), &subject)?;
    TileProduct::from_bytes(&bytes).map_err(|e| err(tile_step(c), &subject, e))
}

/// `terrain.tile`: a level-0 tile from the source (upsample → constrain).
pub struct TileStep(pub Arc<TerrainBake>);

impl Step for TileStep {
    fn id(&self) -> &'static str {
        TILE_STEP
    }
    fn version(&self) -> u32 {
        1
    }
    fn plan(&self, subject: &str, cx: &mut PlanCx) -> Result<(), BakeError> {
        let (ws, c) = subject_of(TILE_STEP, subject)?;
        let plan = self.0.plan(&ws);
        for info in plan.sources_of(c) {
            cx.record(SOURCE_TILE_KIND, source_tile_record(&ws, info.id));
        }
        cx.param("config", self.0.config_bytes());
        cx.param("footprints", self.0.footprints.fingerprint(&ws, c).to_vec());
        cx.param("format", TileProduct::FORMAT.to_le_bytes().to_vec());
        Ok(())
    }
    fn run(&self, subject: &str, _cx: &mut RunCx<'_>) -> Result<Vec<u8>, BakeError> {
        let (ws, c) = subject_of(TILE_STEP, subject)?;
        let bake = &self.0;
        let plan = bake.plan(&ws);
        let mut tiles: Vec<(crate::SourceTileId, SourceTile)> = Vec::new();
        for info in plan.sources_of(c) {
            let tile = bake
                .source
                .read(&ws, info.id)
                .ok_or_else(|| err(TILE_STEP, subject, format!("source tile {:?} is missing", info.id)))?;
            tiles.push((info.id, tile));
        }
        let mut resolve = |id: &RecordId| layer_id(id.as_str());
        let src =
            SourceMosaic::new(tiles, bake.config.fill_height, &mut resolve).map_err(|e| err(TILE_STEP, subject, e))?;
        let mut tile = upsample(c, &src);
        let mut fps = Vec::new();
        bake.footprints.footprints(&ws, c, &mut fps);
        if !fps.is_empty() {
            constrain(&mut tile, &src, &FootprintMask::from_footprints(c, &fps, bake.config.feather));
        }
        let heights = EncodedHeights::encode(&tile);
        // Bounds of what the runtime will see (decoded), not of the f32 tile.
        let bounds = BlockBounds::of(&heights.decode(c));
        let layers = resample_layers(c, &src, &bake.layer_params(&src)).map_err(|e| err(TILE_STEP, subject, e))?;
        let tint = resample_tint(c, &src);
        Ok(TileProduct { coord: c, heights, bounds, layers, tint }.to_bytes())
    }
}

/// `terrain.pyramid`: level k from the level k−1 tiles (even samples,
/// tent-filtered dominant layers, merged bounds).
pub struct PyramidStep(pub Arc<TerrainBake>);

impl PyramidStep {
    /// The 4×4 level k−1 tiles around `c` that exist, as (slot in 0..16, coord).
    fn neighborhood(plan: &TerrainPlan, c: TileCoord) -> impl Iterator<Item = (usize, TileCoord)> + '_ {
        let (x0, z0) = (c.x * 2 - 1, c.z * 2 - 1);
        (0..16usize).filter_map(move |n| {
            let (dx, dz) = (i32::try_from(n % 4).expect("small"), i32::try_from(n / 4).expect("small"));
            let k = TileCoord::new(c.level - 1, x0 + dx, z0 + dz);
            plan.tiles.contains(&k).then_some((n, k))
        })
    }
}

impl Step for PyramidStep {
    fn id(&self) -> &'static str {
        PYRAMID_STEP
    }
    fn version(&self) -> u32 {
        1
    }
    fn plan(&self, subject: &str, cx: &mut PlanCx) -> Result<(), BakeError> {
        let (ws, c) = subject_of(PYRAMID_STEP, subject)?;
        if c.level == 0 {
            return Err(err(PYRAMID_STEP, subject, "level-0 tiles come from terrain.tile"));
        }
        let plan = self.0.plan(&ws);
        for (_, k) in Self::neighborhood(&plan, c) {
            cx.product(tile_step(k), tile_subject(&ws, k));
        }
        cx.param("config", self.0.config_bytes());
        cx.param("format", TileProduct::FORMAT.to_le_bytes().to_vec());
        Ok(())
    }
    fn run(&self, subject: &str, cx: &mut RunCx<'_>) -> Result<Vec<u8>, BakeError> {
        let (ws, c) = subject_of(PYRAMID_STEP, subject)?;
        let plan = self.0.plan(&ws);
        let mut grid: [Option<TileProduct>; 16] = Default::default();
        for (n, k) in Self::neighborhood(&plan, c) {
            grid[n] = Some(read_tile(cx, &ws, k)?);
        }
        // The four children sit at slots 5, 6, 9, 10 of the 4×4 neighborhood.
        let inner = [5, 6, 9, 10];
        let decoded: Vec<Option<crate::HeightTile>> =
            inner.iter().map(|&n| grid[n].as_ref().map(TileProduct::height_tile)).collect();
        let fill = self.0.config.fill_height;
        let tile = downsample(c, [0, 1, 2, 3].map(|i| decoded[i].as_ref()), fill);
        let heights = EncodedHeights::encode(&tile);
        let own = BlockBounds::of(&heights.decode(c));
        let children = inner.map(|n| grid[n].as_ref().map(|t| &t.bounds));
        let bounds = merge_bounds(&own, children, fill);

        let maps: HashMap<TileCoord, &LayerMap> =
            grid.iter().flatten().filter_map(|t| t.layers.as_ref().map(|l| (t.coord, l))).collect();
        let layers = if maps.is_empty() {
            None
        } else {
            let lookup = |k: TileCoord| maps.get(&k).copied();
            let params = LayerParams { blend_radius: 1.0, default_layer: layer_id(&self.0.config.default_layer) };
            Some(downsample_layers(c, &lookup, &params).map_err(|e| err(PYRAMID_STEP, subject, e))?)
        };
        let tints: [Option<&TintMap>; 4] = inner.map(|n| grid[n].as_ref().and_then(|t| t.tint.as_ref()));
        let tint = tints.iter().any(Option::is_some).then(|| downsample_tint(c, tints));
        Ok(TileProduct { coord: c, heights, bounds, layers, tint }.to_bytes())
    }
}

/// Bounds of a level-k tile that also cover its children's (each level-k
/// block is 2×2 child blocks). Missing children count as `fill`.
fn merge_bounds(own: &BlockBounds, children: [Option<&BlockBounds>; 4], fill: f32) -> BlockBounds {
    let mut out = own.clone();
    let half = BLOCKS / 2;
    for bz in 0..BLOCKS {
        for bx in 0..BLOCKS {
            let child = children[(bz / half) * 2 + bx / half];
            let k = bz * BLOCKS + bx;
            let (mut lo, mut hi) = (out.min[k], out.max[k]);
            for (a, b) in [(0, 0), (1, 0), (0, 1), (1, 1)] {
                let (cl, ch) = child.map_or((fill, fill), |c| c.get((bx % half) * 2 + a, (bz % half) * 2 + b));
                lo = lo.min(cl);
                hi = hi.max(ch);
            }
            out.min[k] = lo;
            out.max[k] = hi;
        }
    }
    out
}

/// sRGB u8 -> linear.
pub fn srgb_to_linear(c: u8) -> f32 {
    let c = f32::from(c) / 255.0;
    if c <= 0.04045 { c / 12.92 } else { ((c + 0.055) / 1.055).powf(2.4) }
}

/// Linear -> sRGB u8.
pub fn linear_to_srgb(v: f32) -> u8 {
    let v = v.clamp(0.0, 1.0);
    let s = if v <= 0.003_130_8 { v * 12.92 } else { 1.055 * v.powf(1.0 / 2.4) - 0.055 };
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let b = (s * 255.0).round() as u8;
    b
}

/// `terrain.macro`: the far-terrain albedo of a tile (layer mean albedo
/// weighted by the layer map, times the tint).
pub struct MacroStep(pub Arc<TerrainBake>);

impl Step for MacroStep {
    fn id(&self) -> &'static str {
        MACRO_STEP
    }
    fn version(&self) -> u32 {
        2
    }
    fn plan(&self, subject: &str, cx: &mut PlanCx) -> Result<(), BakeError> {
        let (ws, c) = subject_of(MACRO_STEP, subject)?;
        cx.product(tile_step(c), tile_subject(&ws, c));
        cx.product(LAYER_TABLE_STEP, "");
        cx.param("format", MacroProduct::FORMAT.to_le_bytes().to_vec());
        Ok(())
    }
    fn run(&self, subject: &str, cx: &mut RunCx<'_>) -> Result<Vec<u8>, BakeError> {
        let (ws, c) = subject_of(MACRO_STEP, subject)?;
        let tile = read_tile(cx, &ws, c)?;
        let table =
            LayerTable::from_bytes(&cx.product(LAYER_TABLE_STEP, "")?).map_err(|e| err(MACRO_STEP, subject, e))?;
        Ok(macro_albedo(&tile, &table).to_bytes())
    }
}

/// Neutral gray where a tile has no layers.
const NO_LAYER_ALBEDO: [f32; 3] = [0.18, 0.18, 0.18];

/// The macro albedo of a tile.
pub fn macro_albedo(tile: &TileProduct, table: &LayerTable) -> MacroProduct {
    // Lookup tables: sRGB bytes to linear, and linear (12 bits) back to sRGB.
    let to_linear: [f32; 256] = std::array::from_fn(|c| srgb_to_linear(u8::try_from(c).expect("byte")));
    let to_srgb: Vec<u8> = (0..4096u16).map(|v| linear_to_srgb(f32::from(v) / 4095.0)).collect();
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let encode = |v: f32| to_srgb[(v.clamp(0.0, 1.0) * 4095.0 + 0.5) as usize];
    // Mean albedo per palette slot of this tile.
    let means: HashMap<LayerId, [f32; 3]> = table.layers.iter().map(|l| (l.id, l.mean_albedo)).collect();
    let slot_means: Vec<[f32; 3]> = tile
        .layers
        .as_ref()
        .map(|m| m.palette.iter().map(|id| means.get(id).copied().unwrap_or(NO_LAYER_ALBEDO)).collect())
        .unwrap_or_default();
    // Linear tint on its own 129² grid; layer samples between tint samples interpolate.
    let tint: Option<Vec<[f32; 3]>> =
        tile.tint.as_ref().map(|t| t.rgb.iter().map(|c| c.map(|v| to_linear[usize::from(v)])).collect());
    let n = LAYER_SAMPLES;
    let mut rgba = Vec::with_capacity(n * n);
    for j in 0..n {
        for i in 0..n {
            let mut c = NO_LAYER_ALBEDO;
            if let Some(map) = &tile.layers {
                let k = j * n + i;
                let (s, w) = (map.slots[k], map.weights[k]);
                let w = [w[0], w[1], w[2], 255 - w[0] - w[1] - w[2]];
                c = [0.0; 3];
                for (slot, w) in s.iter().zip(w) {
                    if w > 0 {
                        let m = slot_means[usize::from(*slot)];
                        let f = f32::from(w) / 255.0;
                        for a in 0..3 {
                            c[a] += m[a] * f;
                        }
                    }
                }
            }
            if let Some(t) = &tint {
                let tint = sample_tint(t, i, j);
                for a in 0..3 {
                    c[a] *= tint[a];
                }
            }
            rgba.push([encode(c[0]), encode(c[1]), encode(c[2]), 255]);
        }
    }
    MacroProduct { coord: tile.coord, rgba }
}

/// Tint (linear multiplier) at layer sample (i, j): tint samples sit every
/// second layer sample; odd ones interpolate.
fn sample_tint(t: &[[f32; 3]], i: usize, j: usize) -> [f32; 3] {
    let at = |a: usize, b: usize| t[b.min(TINT_SAMPLES - 1) * TINT_SAMPLES + a.min(TINT_SAMPLES - 1)];
    let (a, fa) = (i / 2, if i % 2 == 1 { 0.5 } else { 0.0 });
    let (b, fb) = (j / 2, if j % 2 == 1 { 0.5 } else { 0.0 });
    let mut out = [0.0; 3];
    for (k, o) in out.iter_mut().enumerate() {
        let top = at(a, b)[k] * (1.0 - fa) + at(a + 1, b)[k] * fa;
        let bottom = at(a, b + 1)[k] * (1.0 - fa) + at(a + 1, b + 1)[k] * fa;
        *o = top * (1.0 - fb) + bottom * fb;
    }
    out
}

/// Tiling rounded so 1024 m holds a whole number of repeats.
pub fn quantize_tiling(tiling_m: f32) -> f32 {
    let repeats = (1024.0 / tiling_m.max(1e-3)).round().max(1.0);
    1024.0 / repeats
}

fn read_layer(
    step: &'static str,
    subject: &str,
    cx: &mut RunCx<'_>,
    layers: &dyn LayerSource,
    id: &RecordId,
) -> Result<TerrainLayer, BakeError> {
    cx.record(LAYER_KIND, id.as_str())?;
    layers.layer(id).ok_or_else(|| err(step, subject, format!("terrain layer {id:?} is missing")))
}

/// `terrain.layer_table`: ids, parameters, and mean colors of every layer.
pub struct LayerTableStep(pub Arc<TerrainBake>);

impl Step for LayerTableStep {
    fn id(&self) -> &'static str {
        LAYER_TABLE_STEP
    }
    fn version(&self) -> u32 {
        2
    }
    fn plan(&self, _subject: &str, cx: &mut PlanCx) -> Result<(), BakeError> {
        for id in self.0.layers.layers() {
            cx.record(LAYER_KIND, id.as_str());
        }
        cx.param("format", LayerTable::FORMAT.to_le_bytes().to_vec());
        cx.param("fill", self.0.config_bytes());
        Ok(())
    }
    fn run(&self, subject: &str, cx: &mut RunCx<'_>) -> Result<Vec<u8>, BakeError> {
        let src = &*self.0.layers;
        let mut table = LayerTable { fill_height: self.0.config.fill_height, ..LayerTable::default() };
        for id in src.layers() {
            let layer = read_layer(LAYER_TABLE_STEP, subject, cx, src, &id)?;
            let bytes = cx.read_file(&layer.albedo_height)?;
            let img = src.decode(&layer.albedo_height, &bytes).map_err(|e| err(LAYER_TABLE_STEP, subject, e))?;
            let (mean_albedo, mean_height) = mean_color(&img);
            let lid = layer_id(id.as_str());
            if let Some(other) = table.layers.iter().find(|l| l.id == lid) {
                return Err(err(
                    LAYER_TABLE_STEP,
                    subject,
                    format!("layers {} and {id:?} hash to the same id", other.record),
                ));
            }
            table.layers.push(LayerInfo {
                record: id.as_str().to_owned(),
                id: lid,
                tiling_m: quantize_tiling(layer.tiling_m),
                triplanar: layer.triplanar,
                displacement_m: layer.displacement_m,
                mean_albedo,
                mean_height,
            });
        }
        let fill = layer_id(&self.0.config.default_layer);
        table.fill_layer = table.layers.iter().any(|l| l.id == fill).then_some(fill);
        Ok(table.to_bytes())
    }
}

/// Mean linear albedo and mean height (alpha) of an image.
fn mean_color(img: &LayerImage) -> ([f32; 3], f32) {
    let mut sum = [0.0f64; 4];
    for px in img.rgba.as_chunks::<4>().0 {
        for a in 0..3 {
            sum[a] += f64::from(srgb_to_linear(px[a]));
        }
        sum[3] += f64::from(px[3]) / 255.0;
    }
    #[allow(clippy::cast_precision_loss)]
    let n = (img.rgba.len() / 4).max(1) as f64;
    #[allow(clippy::cast_possible_truncation)]
    let m = sum.map(|s| (s / n) as f32);
    ([m[0], m[1], m[2]], m[3])
}

/// A file path and its bytes.
type NamedFile = (String, Vec<u8>);

/// `terrain.layers`: the texture arrays at one size (a quality tier).
pub struct LayersStep(pub Arc<TerrainBake>);

#[derive(Clone, Copy, PartialEq, Eq)]
enum MapKind {
    Albedo,
    Normal,
    Linear,
}

impl Step for LayersStep {
    fn id(&self) -> &'static str {
        LAYERS_STEP
    }
    fn version(&self) -> u32 {
        2
    }
    fn plan(&self, subject: &str, cx: &mut PlanCx) -> Result<(), BakeError> {
        subject.parse::<u32>().map_err(|e| err(LAYERS_STEP, subject, e))?;
        cx.product(LAYER_TABLE_STEP, "");
        for id in self.0.layers.layers() {
            cx.record(LAYER_KIND, id.as_str());
        }
        cx.param("format", LayerArrays::FORMAT.to_le_bytes().to_vec());
        Ok(())
    }
    fn run(&self, subject: &str, cx: &mut RunCx<'_>) -> Result<Vec<u8>, BakeError> {
        let size: u32 = subject.parse().map_err(|e| err(LAYERS_STEP, subject, e))?;
        if !size.is_power_of_two() || size < 4 {
            return Err(err(LAYERS_STEP, subject, "array size must be a power of two"));
        }
        let table =
            LayerTable::from_bytes(&cx.product(LAYER_TABLE_STEP, "")?).map_err(|e| err(LAYERS_STEP, subject, e))?;
        let src = &*self.0.layers;
        // Read every file through `cx` (tracked), then decode and resize in parallel.
        let mut jobs: Vec<[Option<NamedFile>; 3]> = Vec::new();
        for info in &table.layers {
            let layer = read_layer(LAYERS_STEP, subject, cx, src, &RecordId::new(&info.record))?;
            let mut read = |p: &Option<String>| -> Result<Option<NamedFile>, BakeError> {
                p.as_ref().map(|p| Ok((p.clone(), cx.read_file(p)?))).transpose()
            };
            jobs.push([read(&Some(layer.albedo_height.clone()))?, read(&layer.normal)?, read(&layer.arm)?]);
        }
        let mips = size.trailing_zeros() + 1;
        let kinds = [MapKind::Albedo, MapKind::Normal, MapKind::Linear];
        let defaults: [[u8; 4]; 3] = [[128, 128, 128, 255], [128, 128, 255, 255], [255, 230, 0, 255]];
        let slices: Vec<[Vec<u8>; 3]> = jobs
            .par_iter()
            .map(|files| {
                let mut out: [Vec<u8>; 3] = Default::default();
                for m in 0..3 {
                    let img = match &files[m] {
                        Some((path, bytes)) => src.decode(path, bytes).map_err(|e| format!("{path}: {e}"))?,
                        None => LayerImage { width: 1, height: 1, rgba: defaults[m].to_vec() },
                    };
                    out[m] = mip_chain(&img, size, kinds[m]);
                }
                Ok(out)
            })
            .collect::<Result<_, String>>()
            .map_err(|e| err(LAYERS_STEP, subject, e))?;
        let mut arrays = LayerArrays { table, size, mips, albedo_height: Vec::new(), normal_ao_roughness: Vec::new() };
        for [a, n, r] in slices {
            arrays.albedo_height.extend(a);
            // One fetch per layer for normal and surface: normal X and Y (Z is
            // reconstructed), AO, roughness. Terrain is dielectric: metallic is dropped.
            for (n, r) in n.as_chunks::<4>().0.iter().zip(r.as_chunks::<4>().0) {
                arrays.normal_ao_roughness.extend_from_slice(&[n[0], n[1], r[0], r[1]]);
            }
        }
        Ok(arrays.to_bytes())
    }
}

/// Resizes `img` to `size`² (bilinear, wrapping: layer textures tile) and
/// appends its full mip chain (2×2 box; albedo averaged in linear space,
/// normals renormalized).
fn mip_chain(img: &LayerImage, size: u32, kind: MapKind) -> Vec<u8> {
    let (w, h) = (img.width.max(1) as usize, img.height.max(1) as usize);
    let n = size as usize;
    let texel = |x: usize, y: usize| -> [f32; 4] {
        let k = ((y % h) * w + (x % w)) * 4;
        let p = &img.rgba[k..k + 4];
        let rgb = |c: u8| if kind == MapKind::Albedo { srgb_to_linear(c) } else { f32::from(c) / 255.0 };
        [rgb(p[0]), rgb(p[1]), rgb(p[2]), f32::from(p[3]) / 255.0]
    };
    // Level 0 in linear float. Downscaling uses an area average, upscaling bilinear.
    let mut level: Vec<[f32; 4]> = Vec::with_capacity(n * n);
    #[allow(clippy::cast_precision_loss)]
    let (sx, sy) = (w as f64 / n as f64, h as f64 / n as f64);
    for y in 0..n {
        for x in 0..n {
            #[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation, clippy::cast_sign_loss)]
            let v = if sx > 1.0 || sy > 1.0 {
                let (x0, y0) = ((x as f64 * sx) as usize, (y as f64 * sy) as usize);
                let (x1, y1) = (((x + 1) as f64 * sx).ceil() as usize, ((y + 1) as f64 * sy).ceil() as usize);
                let mut acc = [0.0f32; 4];
                let mut count = 0.0f32;
                for yy in y0..y1.max(y0 + 1) {
                    for xx in x0..x1.max(x0 + 1) {
                        let t = texel(xx, yy);
                        for a in 0..4 {
                            acc[a] += t[a];
                        }
                        count += 1.0;
                    }
                }
                acc.map(|v| v / count)
            } else {
                let u = (x as f64 + 0.5) * sx - 0.5;
                let v = (y as f64 + 0.5) * sy - 0.5;
                let (fu, fv) = (u - u.floor(), v - v.floor());
                let wrap =
                    |c: f64, n: usize| (c.floor() as i64).rem_euclid(i64::try_from(n).expect("image size")) as usize;
                let (iu, iv) = (wrap(u, w), wrap(v, h));
                let (a, b, c, d) = (texel(iu, iv), texel(iu + 1, iv), texel(iu, iv + 1), texel(iu + 1, iv + 1));
                let (fu, fv) = (fu as f32, fv as f32);
                std::array::from_fn(|k| {
                    (a[k] * (1.0 - fu) + b[k] * fu) * (1.0 - fv) + (c[k] * (1.0 - fu) + d[k] * fu) * fv
                })
            };
            level.push(v);
        }
    }
    let mut out = Vec::new();
    let mut dim = n;
    loop {
        for px in &level {
            let mut p = *px;
            if kind == MapKind::Normal {
                let v =
                    glam::Vec3::new(p[0] * 2.0 - 1.0, p[1] * 2.0 - 1.0, p[2] * 2.0 - 1.0).normalize_or(glam::Vec3::Z);
                p = [v.x * 0.5 + 0.5, v.y * 0.5 + 0.5, v.z * 0.5 + 0.5, p[3]];
            }
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
            let unorm = |v: f32| (v.clamp(0.0, 1.0) * 255.0).round() as u8;
            if kind == MapKind::Albedo {
                out.extend_from_slice(&[linear_to_srgb(p[0]), linear_to_srgb(p[1]), linear_to_srgb(p[2]), unorm(p[3])]);
            } else {
                out.extend_from_slice(&[unorm(p[0]), unorm(p[1]), unorm(p[2]), unorm(p[3])]);
            }
        }
        if dim == 1 {
            break;
        }
        let half = dim / 2;
        let mut next = Vec::with_capacity(half * half);
        for y in 0..half {
            for x in 0..half {
                let s = [
                    level[(2 * y) * dim + 2 * x],
                    level[(2 * y) * dim + 2 * x + 1],
                    level[(2 * y + 1) * dim + 2 * x],
                    level[(2 * y + 1) * dim + 2 * x + 1],
                ];
                next.push(std::array::from_fn(|k| (s[0][k] + s[1][k] + s[2][k] + s[3][k]) * 0.25));
            }
        }
        level = next;
        dim = half;
    }
    out
}

/// `terrain.rt_proxy`: a static grid mesh for ray tracing (Solari can't see
/// shader-morphed CDLOD geometry).
pub struct RtProxyStep(pub Arc<TerrainBake>);

impl Step for RtProxyStep {
    fn id(&self) -> &'static str {
        RT_PROXY_STEP
    }
    fn version(&self) -> u32 {
        1
    }
    fn plan(&self, subject: &str, cx: &mut PlanCx) -> Result<(), BakeError> {
        let (ws, c) = subject_of(RT_PROXY_STEP, subject)?;
        cx.product(tile_step(c), tile_subject(&ws, c));
        cx.param("stride", self.0.config.rt_stride.to_le_bytes().to_vec());
        cx.param("format", RtProxy::FORMAT.to_le_bytes().to_vec());
        Ok(())
    }
    fn run(&self, subject: &str, cx: &mut RunCx<'_>) -> Result<Vec<u8>, BakeError> {
        let (ws, c) = subject_of(RT_PROXY_STEP, subject)?;
        let tile = read_tile(cx, &ws, c)?.height_tile();
        let stride = self.0.config.rt_stride.max(1) as usize;
        if !TILE_QUADS.is_multiple_of(stride) {
            return Err(err(RT_PROXY_STEP, subject, "rt_stride must divide 512"));
        }
        let n = TILE_QUADS / stride + 1;
        let mut heights = Vec::with_capacity(n * n);
        for j in 0..n {
            for i in 0..n {
                heights.push(tile.get(i * stride, j * stride));
            }
        }
        Ok(RtProxy {
            coord: c,
            stride: u32::try_from(stride).expect("small"),
            samples: u32::try_from(n).expect("small"),
            heights,
        }
        .to_bytes())
    }
}

/// Reads one worldspace's baked terrain from a store through a profile
/// manifest: what the runtime streams from.
pub struct BakedTerrain {
    store: sf_bake::Store,
    products: BTreeMap<(String, String), sf_bake::Digest>,
    ws: WorldspaceId,
}

impl BakedTerrain {
    pub fn new(store: sf_bake::Store, manifest: &sf_bake::Manifest, ws: WorldspaceId) -> Self {
        let products = manifest.products.iter().map(|(k, d)| ((k.step.clone(), k.subject.clone()), *d)).collect();
        Self { store, products, ws }
    }

    pub fn worldspace(&self) -> &WorldspaceId {
        &self.ws
    }

    fn bytes(&self, step: &str, subject: &str) -> Option<Result<Vec<u8>, std::io::Error>> {
        let d = self.products.get(&(step.to_owned(), subject.to_owned()))?;
        Some(self.store.get(d))
    }

    /// Whether a canonical tile exists.
    pub fn has_tile(&self, c: TileCoord) -> bool {
        self.products.contains_key(&(tile_step(c).to_owned(), tile_subject(&self.ws, c)))
    }

    /// Every tile of this worldspace.
    pub fn tiles(&self) -> Vec<TileCoord> {
        self.products
            .keys()
            .filter(|(step, _)| step == TILE_STEP || step == PYRAMID_STEP)
            .filter_map(|(_, s)| parse_tile_subject(s).filter(|(ws, _)| *ws == self.ws).map(|(_, c)| c))
            .collect()
    }

    pub fn tile(&self, c: TileCoord) -> Option<Result<TileProduct, String>> {
        let b = self.bytes(tile_step(c), &tile_subject(&self.ws, c))?;
        Some(b.map_err(|e| e.to_string()).and_then(|b| TileProduct::from_bytes(&b).map_err(|e| e.to_string())))
    }

    pub fn macro_albedo(&self, c: TileCoord) -> Option<Result<MacroProduct, String>> {
        let b = self.bytes(MACRO_STEP, &tile_subject(&self.ws, c))?;
        Some(b.map_err(|e| e.to_string()).and_then(|b| MacroProduct::from_bytes(&b).map_err(|e| e.to_string())))
    }

    pub fn rt_proxy(&self, c: TileCoord) -> Option<Result<RtProxy, String>> {
        let b = self.bytes(RT_PROXY_STEP, &tile_subject(&self.ws, c))?;
        Some(b.map_err(|e| e.to_string()).and_then(|b| RtProxy::from_bytes(&b).map_err(|e| e.to_string())))
    }

    /// The layer arrays at `size` (the largest baked one if `None`).
    pub fn layers(&self, size: Option<u32>) -> Option<Result<LayerArrays, String>> {
        let subject = match size {
            Some(s) => s.to_string(),
            None => self
                .products
                .keys()
                .filter(|(step, _)| step == LAYERS_STEP)
                .filter_map(|(_, s)| s.parse::<u32>().ok())
                .max()?
                .to_string(),
        };
        let b = self.bytes(LAYERS_STEP, &subject)?;
        Some(b.map_err(|e| e.to_string()).and_then(|b| LayerArrays::from_bytes(&b).map_err(|e| e.to_string())))
    }
}
