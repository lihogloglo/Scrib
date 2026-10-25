// Seams and pass-through are bitwise; the synthetic world uses plain casts.
#![allow(clippy::float_cmp, clippy::cast_precision_loss, clippy::cast_possible_truncation, clippy::cast_sign_loss)]

use std::collections::{BTreeSet, HashMap};
use std::sync::{Arc, RwLock};

use glam::{DVec3, UVec2};
use sf_bake::{BakeError, Baker, Digest, Inputs, Store};
use sf_coords::WorldPos;
use sf_records::RecordId;
use sf_world::{Fingerprint, WorldspaceId};

use crate::bake::*;
use crate::{
    LayerArrays, LayerTable, MacroProduct, RtProxy, SourceLayers, SourceTile, SourceTileId, SourceTileInfo, SourceTint,
    TILE_SAMPLES, TerrainSource, TileCoord, Triangulation,
};

const QUADS: u32 = 64;
const SPACING: f32 = 2.0;
const N: i64 = 6;

/// A 6×6 grid of 128 m source tiles; each tile's content depends on a
/// version the test can bump.
struct Synthetic {
    versions: RwLock<HashMap<u64, u32>>,
}

impl Synthetic {
    fn new() -> Arc<Self> {
        Arc::new(Self { versions: RwLock::new(HashMap::new()) })
    }
    fn version(&self, id: u64) -> u32 {
        self.versions.read().unwrap().get(&id).copied().unwrap_or(0)
    }
    fn bump(&self, id: u64) {
        *self.versions.write().unwrap().entry(id).or_default() += 1;
    }
    fn origin(tx: i64, tz: i64) -> WorldPos {
        let step = f64::from(QUADS) * f64::from(SPACING);
        WorldPos::from_f64(DVec3::new(-100.0 + tx as f64 * step, 0.0, -60.0 + tz as f64 * step))
    }
}

impl TerrainSource for Synthetic {
    fn tiles(&self, _ws: &WorldspaceId, out: &mut dyn FnMut(SourceTileInfo)) {
        let ext = f32::from(u16::try_from(QUADS).unwrap()) * SPACING;
        for tz in 0..N {
            for tx in 0..N {
                let id = (tz * N + tx) as u64;
                let min = Self::origin(tx, tz);
                let mut fp = [0u8; 32];
                fp[..8].copy_from_slice(&id.to_le_bytes());
                fp[8..12].copy_from_slice(&self.version(id).to_le_bytes());
                out(SourceTileInfo {
                    id: SourceTileId(id),
                    fingerprint: Fingerprint(fp),
                    min,
                    max: min.offset(glam::Vec3::new(ext, 0.0, ext)),
                });
            }
        }
    }

    fn read(&self, _ws: &WorldspaceId, tile: SourceTileId) -> Option<SourceTile> {
        let id = i64::try_from(tile.0).ok()?;
        let (tx, tz) = (id % N, id / N);
        let bump = self.version(tile.0) as f32 * 3.0;
        let n = QUADS + 1;
        let (i0, j0) = (tx * i64::from(QUADS), tz * i64::from(QUADS));
        let mut heights = Vec::new();
        for j in 0..i64::from(n) {
            for i in 0..i64::from(n) {
                let (x, z) = ((i0 + i) as f32, (j0 + j) as f32);
                // Shared borders must agree, so the bump applies inside the tile only.
                let inside = i > 0 && j > 0 && i < i64::from(QUADS) && j < i64::from(QUADS);
                heights.push(20.0 * (x / 17.0).sin() * (z / 23.0).cos() + if inside { bump } else { 0.0 });
            }
        }
        let origin = Self::origin(tx, tz);
        let texels = (0..16).map(|k| u16::from((k + tx + tz) % 3 == 0)).collect();
        Some(SourceTile {
            origin,
            spacing: SPACING,
            size: UVec2::splat(n),
            heights,
            surface: Triangulation::Fixed(crate::Diagonal::Main),
            layers: Some(SourceLayers {
                origin,
                spacing: SPACING * 16.0,
                size: UVec2::splat(4),
                palette: vec![RecordId::new("grass"), RecordId::new("rock")],
                texels,
            }),
            tint: Some(SourceTint {
                origin,
                spacing: SPACING * 8.0,
                size: UVec2::splat(9),
                rgb: vec![[200, 220, 255]; 81],
            }),
        })
    }
}

struct Layers;

impl LayerSource for Layers {
    fn layers(&self) -> Vec<RecordId> {
        vec![RecordId::new("grass"), RecordId::new("rock")]
    }
    fn layer(&self, id: &RecordId) -> Option<TerrainLayer> {
        Some(TerrainLayer {
            albedo_height: format!("{}.raw", id.as_str()),
            normal: None,
            arm: None,
            tiling_m: 3.0,
            triplanar: id.as_str() == "rock",
            displacement_m: 0.05,
            surface: None,
        })
    }
    fn decode(&self, _path: &str, bytes: &[u8]) -> Result<LayerImage, String> {
        Ok(LayerImage { width: 2, height: 2, rgba: bytes.to_vec() })
    }
}

/// Host inputs: fingerprints from the source and the layer files.
struct Host {
    source: Arc<Synthetic>,
}

impl Inputs for Host {
    fn file_fingerprint(&self, path: &str) -> Result<Option<Digest>, BakeError> {
        Ok(Some(Digest::of(&self.read_file(path)?)))
    }
    fn read_file(&self, path: &str) -> Result<Vec<u8>, BakeError> {
        let c = if path.starts_with("grass") { [60, 140, 40, 128] } else { [120, 120, 120, 200] };
        Ok(c.repeat(4))
    }
    fn record_fingerprint(&self, kind: &str, id: &str) -> Result<Option<Digest>, BakeError> {
        Ok(match kind {
            SOURCE_TILE_KIND => {
                let n: u64 = id.rsplit('|').next().unwrap().parse().unwrap();
                let mut fp = None;
                self.source.tiles(&ws(), &mut |info| {
                    if info.id.0 == n {
                        fp = Some(Digest(info.fingerprint.0));
                    }
                });
                fp
            }
            LAYER_KIND => Some(Digest::of(id.as_bytes())),
            _ => None,
        })
    }
}

fn ws() -> WorldspaceId {
    WorldspaceId::new("test")
}

fn config() -> TerrainBakeConfig {
    TerrainBakeConfig { min_top_level: 3, default_layer: "grass".into(), ..TerrainBakeConfig::default() }
}

fn baker(store: &Store, source: &Arc<Synthetic>) -> (Baker, Arc<TerrainBake>) {
    let bake = TerrainBake::new(source.clone(), Arc::new(NoFootprints), Arc::new(Layers), config());
    let mut baker = Baker::new(store.clone(), Arc::new(Host { source: source.clone() }));
    bake.register(&mut baker);
    (baker, bake)
}

/// Per-step counters of one bake session.
struct Report {
    steps: std::collections::BTreeMap<&'static str, sf_bake::StepStats>,
}

fn bake_all(store: &Store, source: &Arc<Synthetic>) -> (Report, sf_bake::Manifest, Arc<TerrainBake>) {
    let (baker, bake) = baker(store, source);
    let failed = bake.bake(&baker, &ws(), 16);
    assert!(failed.is_empty(), "{failed:?}");
    // Every planned product exists, in stage order or not.
    assert_eq!(baker.manifest().products.len(), bake.products(&ws(), 16).len());
    (Report { steps: baker.step_stats() }, baker.manifest(), bake)
}

#[test]
fn changing_one_source_tile_rebuilds_only_its_tiles_and_ancestors() {
    let dir = tempfile::tempdir().unwrap();
    let store = Store::open(dir.path()).unwrap();
    let source = Synthetic::new();
    let (first, _, bake) = bake_all(&store, &source);
    let plan = bake.plan(&ws());
    let level0: Vec<TileCoord> = plan.tiles.iter().copied().filter(|c| c.level == 0).collect();
    assert_eq!(first.steps[TILE_STEP].built as usize, level0.len());
    assert!(plan.top >= 3);

    // Warm: nothing runs.
    let (warm, _, _) = bake_all(&store, &source);
    assert!(warm.steps.values().all(|s| s.built == 0), "{:?}", warm.steps);

    // Change a corner source tile.
    source.bump(0);
    let (after, _, bake) = bake_all(&store, &source);
    let plan = bake.plan(&ws());
    let readers: BTreeSet<TileCoord> =
        level0.iter().copied().filter(|&c| plan.sources_of(c).any(|s| s.id.0 == 0)).collect();
    assert!(!readers.is_empty() && readers.len() < level0.len());
    assert_eq!(after.steps[TILE_STEP].built as usize, readers.len());
    // Pyramid tiles that may rebuild: those whose 4×4 child neighborhood
    // contains a changed tile, level by level.
    let mut changed = readers.clone();
    let mut allowed = 0;
    let pyramid_total = plan.tiles.iter().filter(|c| c.level > 0).count();
    for level in 1..=plan.top {
        let next: BTreeSet<TileCoord> = plan
            .tiles
            .iter()
            .copied()
            .filter(|c| c.level == level)
            .filter(|c| changed.iter().any(|k| (k.x - (c.x * 2 - 1)) as u32 <= 3 && (k.z - (c.z * 2 - 1)) as u32 <= 3))
            .collect();
        allowed += next.len();
        changed = next;
    }
    let rebuilt = after.steps[PYRAMID_STEP].built as usize;
    assert!(rebuilt >= usize::from(plan.top), "the ancestor chain rebuilds ({rebuilt})");
    assert!(rebuilt <= allowed && allowed < pyramid_total, "{rebuilt} rebuilt, {allowed} allowed of {pyramid_total}");
    assert!(after.steps[MACRO_STEP].built as usize <= readers.len() + rebuilt);
    assert_eq!(after.steps[LAYERS_STEP].built, 0);
    assert_eq!(after.steps[LAYER_TABLE_STEP].built, 0);
}

#[test]
fn bakes_are_deterministic_across_thread_counts() {
    let source = Synthetic::new();
    let run = |threads: usize| {
        let dir = tempfile::tempdir().unwrap();
        let store = Store::open(dir.path()).unwrap();
        let pool = rayon::ThreadPoolBuilder::new().num_threads(threads).build().unwrap();
        pool.install(|| bake_all(&store, &source).1)
    };
    assert_eq!(run(1), run(6));
}

#[test]
fn products_decode_and_agree_on_seams_and_levels() {
    let dir = tempfile::tempdir().unwrap();
    let store = Store::open(dir.path()).unwrap();
    let source = Synthetic::new();
    let (_, manifest, bake) = bake_all(&store, &source);
    let baked = BakedTerrain::new(store, &manifest, ws());
    let plan = bake.plan(&ws());
    assert_eq!(baked.tiles().len(), plan.tiles.len());

    let tile = |c: TileCoord| baked.tile(c).unwrap().unwrap();
    // Seams between level-0 neighbors are bitwise equal.
    let (a, b) = (TileCoord::new(0, 0, 0), TileCoord::new(0, 1, 0));
    let (ha, hb) = (tile(a).height_tile(), tile(b).height_tile());
    for j in 0..TILE_SAMPLES {
        assert_eq!(ha.get(TILE_SAMPLES - 1, j), hb.get(0, j));
    }
    // Level 1 = level 0 at even indices; bounds cover the children.
    let p = TileCoord::new(1, 0, 0);
    let (hp, tp) = (tile(p).height_tile(), tile(p));
    let tol = tp.heights.max_error() + tile(a).heights.max_error() + 1e-6;
    for j in 0..=256 {
        for i in 0..=256 {
            // Each level is quantized on its own (≤ half a step each); the
            // renderer blends to the coarse level where LODs meet.
            assert!((hp.get(i, j) - ha.get(2 * i, 2 * j)).abs() <= tol);
        }
    }
    let ta = tile(a);
    for bz in 0..16 {
        for bx in 0..16 {
            let (lo, hi) = tp.bounds.get(bx, bz);
            for (dx, dz) in [(0, 0), (1, 0), (0, 1), (1, 1)] {
                let (clo, chi) = ta.bounds.get(bx * 2 + dx, bz * 2 + dz);
                assert!(lo <= clo && hi >= chi);
            }
        }
    }
    assert!(tp.layers.is_some() && tp.tint.is_some());

    let table: &LayerTable = &baked.layers(Some(16)).unwrap().unwrap().table;
    assert_eq!(table.layers.len(), 2);
    assert_eq!(table.layers[0].tiling_m, 1024.0 / 341.0);
    let arrays: LayerArrays = baked.layers(None).unwrap().unwrap();
    assert_eq!(arrays.mips, 5);
    let per_slice: usize = (0..arrays.mips).map(|m| (16usize >> m).pow(2) * 4).sum();
    assert_eq!(arrays.albedo_height.len(), per_slice * 2);

    let m: MacroProduct = baked.macro_albedo(a).unwrap().unwrap();
    assert!(m.rgba.iter().all(|c| c[3] == 255));
    let r: RtProxy = baked.rt_proxy(a).unwrap().unwrap();
    assert_eq!(r.samples, 129);
    assert_eq!(r.heights[1], ha.get(4, 0));
    let (pos, nrm, idx) = r.mesh();
    assert_eq!(pos.len(), 129 * 129);
    assert_eq!(nrm.len(), pos.len());
    assert_eq!(idx.len(), 128 * 128 * 6);
}

#[test]
fn subjects_round_trip() {
    let c = TileCoord::new(3, -7, 12);
    let ws = WorldspaceId::new("a|b");
    assert_eq!(parse_tile_subject(&tile_subject(&ws, c)), Some((ws, c)));
}

#[test]
fn placement_footprints_cover_grounded_placements_only() {
    use sf_coords::WorldTransform;
    use sf_world::{Placement, PlacementClass, PlacementFlags, PlacementId, RecordRef};
    let place = |x: f64, flags| Placement {
        id: PlacementId(1),
        base: RecordRef::new("static", "rock"),
        model: None,
        transform: WorldTransform {
            pos: WorldPos::from_f64(DVec3::new(x, 0.0, 10.0)),
            rotation: glam::Quat::from_rotation_y(0.5),
            scale: glam::Vec3::splat(2.0),
        },
        class: PlacementClass::Static,
        flags,
    };
    let ps = [
        place(10.0, PlacementFlags::GROUNDED),
        place(50.0, PlacementFlags::empty()),
        place(127.0, PlacementFlags::GROUNDED),
    ];
    let mut f = PlacementFootprints::new();
    let bounds = |_: &Placement| Some((glam::Vec3::new(-1.0, 0.0, -2.0), glam::Vec3::new(1.0, 3.0, 2.0)));
    f.set_worldspace(&ws(), &ps, &bounds, 1.5);
    let mut out = Vec::new();
    f.footprints(&ws(), TileCoord::new(0, 0, 0), &mut out);
    assert_eq!(out.len(), 2);
    assert!((out[0].yaw - 0.5).abs() < 1e-5);
    assert_eq!(out[0].half_extents, glam::Vec2::new(2.0, 4.0));
    // The one near x = 127 m also reaches the next tile (border at 128 m).
    out.clear();
    f.footprints(&ws(), TileCoord::new(0, 1, 0), &mut out);
    assert_eq!(out.len(), 1);
    assert_ne!(f.fingerprint(&ws(), TileCoord::new(0, 0, 0)), f.fingerprint(&ws(), TileCoord::new(0, 1, 0)));
    assert_eq!(f.fingerprint(&ws(), TileCoord::new(0, 5, 5)), [0; 32]);
}
