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
    fn decode(&self, _path: &str, bytes: &[u8], _file: LayerFile) -> Result<LayerImage, String> {
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
    // 2² sources under a 16² subject: never upsampled past the largest source (one block, 4²).
    assert_eq!((arrays.size, arrays.mips), (4, 3));
    assert_eq!(table.layers[0].source_edge, 2);
    // Three levels of one 16-byte block each, two layers, in each of the three arrays.
    for a in [&arrays.albedo_height, &arrays.normal, &arrays.ao_roughness] {
        assert_eq!(a.len(), 3 * 16 * 2);
    }

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

/// A height field over (column, row) in texel units.
type Height = fn(f32, f32) -> f32;

/// The world normal `sf_terrain_bevy`'s `surface.wesl` shades for one layer-array texel on the
/// top-down projection: `sample_projection` (x, y from R, G; z rebuilt) then `sample_layer`'s
/// whiteout blend onto the ground normal (`ty = (t.xy + n.xz, |t.z| n.y)`, world = `ty.xzy`).
fn shade_top(nar: [u8; 4], ground: glam::Vec3) -> glam::Vec3 {
    let x = f32::from(nar[0]) / 255.0 * 2.0 - 1.0;
    let y = f32::from(nar[1]) / 255.0 * 2.0 - 1.0;
    let z = (1.0 - x * x - y * y).max(0.0).sqrt();
    let ty = glam::Vec3::new(x + ground.x, y + ground.z, z.abs() * ground.y);
    glam::Vec3::new(ty.x, ty.z, ty.y).normalize()
}

/// A canonical (+Y toward the image's top) normal map of height `h(col, row)` (texel units).
fn canonical_normals(n: usize, h: impl Fn(f32, f32) -> f32) -> LayerImage {
    let mut rgba = Vec::new();
    for row in 0..n {
        for col in 0..n {
            #[allow(clippy::cast_precision_loss)]
            let (c, r) = (col as f32, row as f32);
            let dx = h(c + 0.5, r) - h(c - 0.5, r);
            // The image's up is −row.
            let dy_up = -(h(c, r + 0.5) - h(c, r - 0.5));
            let v = glam::Vec3::new(-dx, -dy_up, 1.0).normalize();
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
            let u = |f: f32| ((f * 0.5 + 0.5) * 255.0).round() as u8;
            rgba.extend_from_slice(&[u(v.x), u(v.y), u(v.z), 255]);
        }
    }
    let n = u32::try_from(n).unwrap();
    LayerImage { width: n, height: n, rgba }
}

/// T5 tangent frame: a canonical normal map baked into the arrays and shaded the way the shader
/// does gives the geometric normal of the relief its albedo shows (top-down projection: texel
/// column = world +X, row = world +Z), so bumps aren't inverted.
#[test]
fn layer_normals_shade_the_relief_the_albedo_shows() {
    let slope = 0.5;
    let ground = glam::Vec3::Y;
    // Height rising down the rows (world +Z): y = slope·z faces −Z.
    // Height rising along the columns (world +X): y = slope·x faces −X.
    let cases: [(Height, glam::Vec3); 2] = [
        (|_, r| 0.5 * r, glam::Vec3::new(0.0, 1.0, -slope)),
        (|c, _| 0.5 * c, glam::Vec3::new(-slope, 1.0, 0.0)),
    ];
    for (h, expected) in cases {
        let mut img = canonical_normals(4, h);
        normal_to_array_frame(&mut img.rgba);
        let arrays = mip_chain(&img, 4, MapKind::Normal);
        let t = &arrays[5 * 4..5 * 4 + 4];
        let world = shade_top([t[0], t[1], 0, 0], ground);
        assert!(world.abs_diff_eq(expected.normalize(), 0.01), "{world} vs {}", expected.normalize());
    }
}

/// The arrays' frame complements G and rebuilds Z (a two-channel source has none).
#[test]
fn array_frame_flips_green_and_rebuilds_z() {
    let mut rgba = vec![128, 200, 0, 7, 255, 128, 99, 255];
    normal_to_array_frame(&mut rgba);
    assert_eq!(rgba[..2], [128, 55]);
    assert_eq!(rgba[3], 7);
    // (0.004, −0.569) → z 0.823 → 232.
    assert_eq!(rgba[2], 232);
    assert_eq!(rgba[4..8], [255, 127, 128, 255]);
}

/// Decodes level `mip` of slice `slice` of one array (`LayerArrays` layout).
fn decode_array_level(data: &[u8], format: sf_texture::BlockFormat, size: u32, mips: u32, slice: usize, mip: u32) -> Vec<u8> {
    let start = slice * LayerArrays::slice_len(size, mips) + (0..mip).map(|m| LayerArrays::level_len(size, m)).sum::<usize>();
    let dim = (size >> mip).max(1);
    sf_texture::decode_level(format, &data[start..start + LayerArrays::level_len(size, mip)], dim, dim)
}

/// Tileable test pattern with height in alpha.
fn pattern(n: u32) -> LayerImage {
    let mut rgba = Vec::new();
    for y in 0..n {
        for x in 0..n {
            let (u, v) = (x as f32 / n as f32 * std::f32::consts::TAU, y as f32 / n as f32 * std::f32::consts::TAU);
            let w = |f: f32| ((f.sin() * 0.5 + 0.5) * 200.0 + 20.0) as u8;
            rgba.extend_from_slice(&[w(u), w(v), w(u + v), w(2.0 * u - v)]);
        }
    }
    LayerImage { width: n, height: n, rgba }
}

/// T6: the three arrays' sizes: every level is whole 16-byte blocks (levels under 4² take one),
/// the chain runs to 1², BC7 albedo + height decodes back to the RGBA8 chain within BC7's error,
/// and AO + roughness are the ARM map's R and G.
#[test]
fn layer_arrays_are_block_compressed_full_chains() {
    assert_eq!(LayerArrays::level_len(64, 0), 16 * 16 * 16);
    assert_eq!(LayerArrays::level_len(64, 5), 16);
    assert_eq!(LayerArrays::level_len(64, 6), 16);
    assert_eq!(LayerArrays::slice_len(64, 7), 16 * (256 + 64 + 16 + 4 + 1 + 1 + 1));
    let img = pattern(64);
    let [a, n, r] = encode_layer(Some(&img), Some(&img), Some(&img), 64);
    for data in [&a, &n, &r] {
        assert_eq!(data.len(), LayerArrays::slice_len(64, 7));
    }
    let chain = mip_chain(&img, 64, MapKind::Albedo);
    let decoded = decode_array_level(&a, sf_texture::BlockFormat::Bc7Srgb, 64, 7, 0, 0);
    // Four uncorrelated channels are BC7's hard case: mean and worst error stay small.
    let diffs: Vec<u8> = chain[..64 * 64 * 4].iter().zip(&decoded).map(|(x, y)| x.abs_diff(*y)).collect();
    let mean = diffs.iter().map(|&d| f64::from(d)).sum::<f64>() / diffs.len() as f64;
    let worst = diffs.iter().copied().max().unwrap();
    assert!(mean <= 4.0 && worst <= 24, "BC7 error mean {mean:.2}, worst {worst}");
    let ar = decode_array_level(&r, sf_texture::BlockFormat::Bc5, 64, 7, 0, 0);
    let lin = mip_chain(&img, 64, MapKind::Linear);
    for (t, l) in ar.as_chunks::<4>().0.iter().zip(lin.as_chunks::<4>().0) {
        assert!(t[0].abs_diff(l[0]) <= 4 && t[1].abs_diff(l[1]) <= 4, "{t:?} vs {l:?}");
    }
}

/// T6: missing maps keep their defaults exactly through compression, on every level.
#[test]
fn missing_maps_keep_their_defaults() {
    use sf_texture::BlockFormat;
    let size = 16;
    let maps = encode_layer(None, None, None, size);
    let formats = [BlockFormat::Bc7Srgb, BlockFormat::Bc5, BlockFormat::Bc5];
    for ((data, format), default) in maps.iter().zip(formats).zip(LAYER_DEFAULTS) {
        assert_eq!(data.len(), LayerArrays::slice_len(size, 5));
        for mip in 0..5 {
            let texels = decode_array_level(data, format, size, 5, 0, mip);
            let want: &[u8] = if format == BlockFormat::Bc5 { &default[..2] } else { &default };
            for t in texels.as_chunks::<4>().0 {
                assert_eq!(&t[..want.len()], want, "{format:?} level {mip}");
            }
        }
    }
}

/// T6 + T5: the relief direction survives BC5: a canonical normal map baked and compressed, then
/// shaded the way the shader does, still gives the relief's geometric normal.
#[test]
fn bc5_layer_normals_shade_the_relief_the_albedo_shows() {
    let slope = 0.5;
    let cases: [(Height, glam::Vec3); 2] = [
        (|_, r| 0.5 * r, glam::Vec3::new(0.0, 1.0, -slope)),
        (|c, _| 0.5 * c, glam::Vec3::new(-slope, 1.0, 0.0)),
    ];
    for (h, expected) in cases {
        let mut img = canonical_normals(16, h);
        normal_to_array_frame(&mut img.rgba);
        let [_, n, _] = encode_layer(None, Some(&img), None, 16);
        let texels = decode_array_level(&n, sf_texture::BlockFormat::Bc5, 16, 5, 0, 0);
        for t in texels.as_chunks::<4>().0 {
            let world = shade_top([t[0], t[1], 0, 0], glam::Vec3::Y);
            assert!(world.abs_diff_eq(expected.normalize(), 0.02), "{world} vs {}", expected.normalize());
        }
    }
}

/// Arrays whose every byte names its slice and level, `size`² with a full chain.
fn tagged_arrays(layers: usize, size: u32) -> LayerArrays {
    let mips = size.trailing_zeros() + 1;
    let mut data = Vec::new();
    for slice in 0..layers {
        for mip in 0..mips {
            data.extend(std::iter::repeat_n((slice * 32) as u8 + mip as u8, LayerArrays::level_len(size, mip)));
        }
    }
    let info = |k: usize| crate::LayerInfo {
        record: format!("l{k}"),
        id: crate::layer_id(&format!("l{k}")),
        tiling_m: 4.0,
        triplanar: false,
        displacement_m: 0.0,
        mean_albedo: [0.5; 3],
        mean_height: 1.0,
        source_edge: size,
    };
    let table = LayerTable { layers: (0..layers).map(info).collect(), ..LayerTable::default() };
    LayerArrays { table, size, mips, albedo_height: data.clone(), normal: data.clone(), ao_roughness: data }
}

/// T6: a capped load drops exactly the top levels of every slice of the three arrays.
#[test]
fn layer_cap_drops_exactly_the_top_levels() {
    use crate::LayerCap;
    let full = tagged_arrays(3, 64);
    let bytes = full.to_bytes();
    assert_eq!(LayerArrays::from_bytes(&bytes).unwrap(), full);
    let cap = LayerCap { max_edge: Some(16), budget: None };
    assert_eq!(cap.skip(64, 7, 3), 2);
    let capped = LayerArrays::from_bytes_capped(&bytes, &cap).unwrap();
    assert_eq!((capped.size, capped.mips, capped.table.layers.len()), (16, 5, 3));
    let mut expected = Vec::new();
    for slice in 0..3usize {
        for mip in 2..7u32 {
            expected.extend(std::iter::repeat_n((slice * 32) as u8 + mip as u8, LayerArrays::level_len(64, mip)));
        }
    }
    for a in [&capped.albedo_height, &capped.normal, &capped.ao_roughness] {
        assert_eq!(a, &expected);
    }
    assert_eq!(capped.gpu_bytes(), 3 * 3 * LayerArrays::slice_len(16, 5));
    // A cap at or above the size keeps everything; one below a block stops at one block (4²).
    assert_eq!(LayerCap { max_edge: Some(64), budget: None }.skip(64, 7, 3), 0);
    assert_eq!(LayerCap { max_edge: Some(1), budget: None }.skip(64, 7, 3), 4);
    // The budget drops levels until the three arrays of three layers fit.
    let fits_32 = 3 * 3 * LayerArrays::slice_len(32, 6);
    assert_eq!(LayerCap { max_edge: None, budget: Some(fits_32) }.skip(64, 7, 3), 1);
    assert_eq!(LayerCap { max_edge: None, budget: Some(fits_32 - 1) }.skip(64, 7, 3), 2);
    assert_eq!(LayerCap { max_edge: Some(16), budget: Some(fits_32) }.skip(64, 7, 3), 2);
}

/// T6 measurement: encode time of 2048² layers with albedo, normal and ARM all present (the worst
/// case) on this machine, with an opaque albedo (no height: BC7's opaque modes) and with height in
/// alpha, and the arrays' size per layer at each edge.
/// `cargo test --release -p sf_terrain --features bake --lib -- --ignored --nocapture layer_encode_time`
#[test]
#[ignore = "measurement: terrain.layers encode time at 2048²"]
fn layer_encode_time() {
    use rayon::prelude::*;
    let layers = 16;
    for opaque in [true, false] {
        let mut img = pattern(2048);
        if opaque {
            img.rgba.as_chunks_mut::<4>().0.iter_mut().for_each(|t| t[3] = 255);
        }
        let t = std::time::Instant::now();
        let out: Vec<[Vec<u8>; 3]> = (0..layers).into_par_iter().map(|_| encode_layer(Some(&img), Some(&img), Some(&img), 2048)).collect();
        let s = t.elapsed().as_secs_f64();
        assert_eq!(out.len(), layers);
        println!(
            "terrain.layers encode ({}): {layers} layers x 2048² (BC7 + 2 x BC5, full chains) in {s:.2} s, {:.3} s/layer on {} threads",
            if opaque { "opaque albedo" } else { "height in alpha" },
            s / layers as f64,
            rayon::current_num_threads()
        );
    }
    for edge in [2048u32, 1024, 512, 256] {
        let mib = 3.0 * LayerArrays::slice_len(edge, edge.trailing_zeros() + 1) as f64 / 1_048_576.0;
        println!("  {edge}²: {mib:.2} MiB per layer (three arrays)");
    }
}
