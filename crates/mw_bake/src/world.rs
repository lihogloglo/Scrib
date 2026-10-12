//! The exterior's world index and terrain products (specs:
//! `superfargoth/docs/specs/streaming.md` "Index building",
//! `terrain.md`, `docs/specs/world-adapter.md`).
//!
//! The model steps run first (the index reads `model.info`); then
//! `MwWorldSource` is built with `model.info` bounds for grounding, the
//! `world.*` steps index the exterior, and the `terrain.*` steps bake LAND
//! with the grounded placements' footprints (`constrain`) and the LTEX
//! layers. Everything resolves in one `Baker` session, so one manifest
//! names every product.
//!
//! Interiors are indexed too, each as its own worldspace with the partition
//! its `Worldspace` record names (`Single`: an interior loads whole), so
//! door links resolve to a baked directory (streaming.md, "Worldspaces at
//! runtime").

use std::collections::{BTreeMap, HashMap};
use std::sync::Arc;
use std::time::{Duration, Instant};

use glam::Vec3;
use mw_world::{MwTerrainSource, MwWorldSource, SourceOptions, TerrainLayerDef, ids, terrain};
use sf_bake::{BakeError, Baker, Digest, ProductKey};
use sf_model::steps::ModelInfoStep;
use sf_records::{RecordDb, RecordId};
use sf_terrain::bake::{
    LAYER_KIND, LayerImage, LayerSource, PlacementFootprints, SOURCE_TILE_KIND, TerrainBake,
    TerrainBakeConfig, TerrainLayer, source_tile_record,
};
use sf_terrain::{FOOTPRINT_FEATHER, TerrainSource};
use sf_vfs::Vfs;
use sf_world::bake::WorldBake;
use sf_world::{IndexParams, WorldspaceId};

use crate::textures::decode_rgba;

/// Edge of the terrain layer texture arrays. Vanilla land textures are
/// 256²; larger replacers are downsampled to this (a quality tier later).
pub const LAYER_ARRAY_SIZE: u32 = 256;

/// The exterior's terrain layers as an `sf_terrain` [`LayerSource`].
pub struct MwLayers {
    layers: BTreeMap<RecordId, TerrainLayer>,
}

impl MwLayers {
    /// Every live `LTEX` plus the default layer, with paths resolved through
    /// `vfs`. A layer whose texture is missing (vanilla Tribunal has one)
    /// draws the default land texture, as an unresolved index does.
    pub fn new(db: &RecordDb, vfs: &Vfs) -> Self {
        let defs = terrain::terrain_layers(db, Some(vfs));
        let fallback = defs.iter().find(|(id, _)| id.as_str() == terrain::DEFAULT_LAYER).map(|(_, d)| d.albedo_height.clone());
        let layers = defs
            .into_iter()
            .map(|(id, mut def)| {
                if !vfs.contains(&def.albedo_height)
                    && let Some(f) = &fallback
                {
                    def.albedo_height = f.clone();
                }
                (id, layer_of(&def))
            })
            .collect();
        Self { layers }
    }

    /// Record fingerprints of every layer (`terrain.layer` inputs).
    pub fn fingerprints(&self) -> impl Iterator<Item = ((String, String), Digest)> + '_ {
        self.layers
            .iter()
            .map(|(id, l)| ((LAYER_KIND.to_owned(), id.as_str().to_owned()), Digest::of(format!("{l:?}").as_bytes())))
    }

    pub fn len(&self) -> usize {
        self.layers.len()
    }

    pub fn is_empty(&self) -> bool {
        self.layers.is_empty()
    }
}

/// Field-by-field conversion (world-adapter.md: `TerrainLayerDef` scaffold).
fn layer_of(def: &TerrainLayerDef) -> TerrainLayer {
    TerrainLayer {
        albedo_height: def.albedo_height.as_str().to_owned(),
        normal: def.normal.as_ref().map(|p| p.as_str().to_owned()),
        arm: def.arm.as_ref().map(|p| p.as_str().to_owned()),
        tiling_m: def.tiling_m,
        triplanar: def.triplanar,
        displacement_m: def.displacement_m,
        surface: Some(def.surface.id.clone()),
    }
}

impl LayerSource for MwLayers {
    fn layers(&self) -> Vec<RecordId> {
        self.layers.keys().cloned().collect()
    }

    fn layer(&self, id: &RecordId) -> Option<TerrainLayer> {
        self.layers.get(id).cloned()
    }

    fn decode(&self, path: &str, bytes: &[u8]) -> Result<LayerImage, String> {
        let img = decode_rgba(path, bytes)?;
        Ok(LayerImage { width: img.width, height: img.height, rgba: img.rgba })
    }
}

/// Record fingerprints of every exterior source tile (`terrain.source_tile` inputs).
pub fn source_tile_fingerprints<D>(source: &MwTerrainSource<D>) -> Vec<((String, String), Digest)>
where
    D: std::ops::Deref<Target = RecordDb>,
{
    let ws = ids::exterior();
    source
        .infos()
        .iter()
        .map(|info| ((SOURCE_TILE_KIND.to_owned(), source_tile_record(&ws, info.id)), Digest(info.fingerprint.0)))
        .collect()
}

/// Terrain bake settings for Morrowind: the sea floor where no `LAND`
/// exists, the default texture where none is named.
///
/// Blend radius: a tent of ¾ texel around each texel center leaves the
/// middle half of every texel one texture and ramps over the other half,
/// the transition width of the original engine's blend maps (doubled
/// nearest-neighbor texels, bilinearly filtered). One texel (bilinear)
/// washes roads and paving out.
pub fn terrain_config() -> TerrainBakeConfig {
    TerrainBakeConfig {
        fill_height: terrain::fill_height(),
        blend_radius: Some(0.75 * mw_world::convert::meters(terrain::TEXEL_UNITS)),
        default_layer: terrain::DEFAULT_LAYER.to_owned(),
        ..TerrainBakeConfig::default()
    }
}

/// Model-space bounds of every model `model.info` resolved this session.
pub fn model_bounds(baker: &Baker, models: &[String]) -> HashMap<String, ([f32; 3], [f32; 3])> {
    use rayon::prelude::*;
    // Thousands of small store reads: parallel, or this is I/O-latency bound.
    models
        .par_iter()
        .filter_map(|m| {
            let digest = baker.product(&ProductKey::new(ModelInfoStep::ID, m.as_str())).ok()?;
            let info = ModelInfoStep::decode(&baker.store().get(&digest).ok()?).ok()?;
            (!info.bounds.is_empty()).then(|| (m.clone(), (info.bounds.min, info.bounds.max)))
        })
        .collect()
}

/// What [`bake_exterior`] did.
#[derive(Debug, Default)]
pub struct ExteriorReport {
    pub bounds: Duration,
    pub source_build: Duration,
    pub index: Duration,
    /// Interior worldspaces indexed, and the time it took.
    pub interiors: usize,
    pub interior_index: Duration,
    /// Door links, and link ends whose worldspace has no baked directory.
    pub door_links: usize,
    pub unresolved_link_ends: usize,
    pub terrain: Duration,
    pub placements: usize,
    pub grounded: usize,
    pub terrain_products: usize,
    pub failures: Vec<(ProductKey, Arc<BakeError>)>,
}

/// Registers the world and terrain steps on `baker` (which already has the
/// model steps and their products for `models`) and bakes the exterior.
pub fn bake_exterior(
    baker: &mut Baker,
    db: &RecordDb,
    terrain_source: Arc<dyn TerrainSource>,
    layers: Arc<MwLayers>,
    models: &[String],
) -> ExteriorReport {
    let mut report = ExteriorReport::default();
    let ext = ids::exterior();

    let t = Instant::now();
    let bounds = model_bounds(baker, models);
    report.bounds = t.elapsed();
    let t = Instant::now();
    let lookup = |m: &str| bounds.get(m).copied();
    let source = Arc::new(MwWorldSource::build(db, &SourceOptions { model_bounds: Some(&lookup), ..SourceOptions::default() }));
    report.placements = source.stats().placements;
    report.grounded = source.stats().grounded;
    report.source_build = t.elapsed();

    let t = Instant::now();
    let records = mw_world::WorldRecords::build(db);
    let (worldspaces, overrides) = index_plan(&records, &source);
    WorldBake::with_overrides(source.clone(), IndexParams::default(), overrides).register(baker);
    let index = WorldBake::bake_worldspaces(baker, std::slice::from_ref(&ext));
    report.failures.extend(index.failures().map(|(k, e)| (k.clone(), e.clone())));
    report.index = t.elapsed();

    let t = Instant::now();
    let interiors: Vec<WorldspaceId> = worldspaces.into_iter().filter(|ws| *ws != ext).collect();
    let index = WorldBake::bake_worldspaces(baker, &interiors);
    report.failures.extend(index.failures().map(|(k, e)| (k.clone(), e.clone())));
    report.interiors = interiors.len();
    report.interior_index = t.elapsed();
    report.door_links = records.door_links.len();
    report.unresolved_link_ends = unresolved_link_ends(baker, &records, &interiors);

    let t = Instant::now();
    let mut footprints = PlacementFootprints::new();
    let bounds_of = |p: &sf_world::Placement| {
        let (min, max) = bounds.get(p.model.as_ref()?.as_str())?;
        Some((Vec3::from_array(*min), Vec3::from_array(*max)))
    };
    footprints.set_worldspace(&ext, source.all_placements(&ext), &bounds_of, FOOTPRINT_FEATHER);
    let bake = TerrainBake::new(terrain_source, Arc::new(footprints), layers, terrain_config());
    bake.register(baker);
    report.terrain_products = bake.products(&ext, LAYER_ARRAY_SIZE).len();
    report.failures.extend(bake.bake(baker, &ext, LAYER_ARRAY_SIZE));
    report.terrain = t.elapsed();
    report
}

/// The exterior worldspace id (re-exported for hosts).
pub fn exterior() -> WorldspaceId {
    ids::exterior()
}

/// Every worldspace to index (the exterior first, then interiors by id) and
/// the index settings of those whose `Worldspace` record names another
/// partition than the default (interiors: `Single`).
pub fn index_plan(records: &mw_world::WorldRecords, source: &MwWorldSource) -> (Vec<WorldspaceId>, BTreeMap<WorldspaceId, IndexParams>) {
    let defaults = IndexParams::default();
    let mut overrides = BTreeMap::new();
    let mut worldspaces: Vec<WorldspaceId> = Vec::with_capacity(records.worldspaces.len());
    for (id, ws) in &records.worldspaces {
        if ws.partition != defaults.partition {
            overrides.insert(id.clone(), IndexParams { partition: ws.partition, ..defaults });
        }
        worldspaces.push(id.clone());
    }
    // Worldspaces with placements but no record (none in vanilla) still index.
    for ws in source.worldspaces() {
        if !worldspaces.contains(ws) {
            worldspaces.push(ws.clone());
        }
    }
    let ext = ids::exterior();
    worldspaces.sort_by(|a, b| (*a != ext).cmp(&(*b != ext)).then_with(|| a.cmp(b)));
    worldspaces.dedup();
    (worldspaces, overrides)
}

/// Door link ends (both ends of two-way links, the destination of one-way
/// ones) whose worldspace was not indexed (the exterior or one of
/// `interiors`) or has no `world.dir` product this session.
pub fn unresolved_link_ends(baker: &Baker, records: &mw_world::WorldRecords, interiors: &[WorldspaceId]) -> usize {
    let indexed: std::collections::HashSet<&WorldspaceId> = interiors.iter().collect();
    let ext = ids::exterior();
    let mut known: HashMap<&WorldspaceId, bool> = HashMap::new();
    let mut unresolved = 0;
    for (_, link) in &records.door_links {
        let ends = if link.two_way { vec![&link.a, &link.b] } else { vec![&link.b] };
        for end in ends {
            let ok = *known.entry(&end.worldspace).or_insert_with(|| {
                (end.worldspace == ext || indexed.contains(&end.worldspace))
                    && baker.product(&WorldBake::directory_key(&end.worldspace)).is_ok()
            });
            unresolved += usize::from(!ok);
        }
    }
    unresolved
}
