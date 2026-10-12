//! A procedural world for terrain examples and benchmarks: fBm mountains,
//! valleys and a coast as a `TerrainSource`, five procedural layers, and an
//! in-memory bake host. Shared by `sf_terrain`'s `bake_synthetic` example
//! and `sf_terrain_bevy`'s `terrain` example (included by path).
#![allow(
    dead_code,
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_possible_wrap,
    clippy::too_many_lines,
    clippy::many_single_char_names
)]

use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;

use glam::{DVec3, UVec2, Vec3};
use sf_bake::{BakeError, Baker, Digest, Inputs, Store};
use sf_coords::WorldPos;
use sf_records::RecordId;
use sf_terrain::bake::{
    BakedTerrain, LAYER_KIND, LayerImage, LayerSource, NoFootprints, SOURCE_TILE_KIND, TerrainBake, TerrainBakeConfig,
    TerrainLayer,
};
use sf_terrain::{
    Diagonal, SourceLayers, SourceTile, SourceTileId, SourceTileInfo, SourceTint, TerrainSource, Triangulation,
};
use sf_world::{Fingerprint, WorldspaceId};

// ---------------------------------------------------------------- noise

fn hash(i: i64, j: i64, seed: u64) -> f64 {
    let mut x = (i as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15)
        ^ (j as u64).wrapping_mul(0xC2B2_AE3D_27D4_EB4F)
        ^ seed.wrapping_mul(0x1656_67B1_9E37_79F9);
    x ^= x >> 31;
    x = x.wrapping_mul(0xBF58_476D_1CE4_E5B9);
    x ^= x >> 29;
    (x >> 11) as f64 / (1u64 << 53) as f64
}

/// Value noise in [0, 1], optionally periodic (for tileable textures).
fn noise(x: f64, z: f64, seed: u64, period: Option<i64>) -> f64 {
    let (i, j) = (x.floor() as i64, z.floor() as i64);
    let (fx, fz) = (x - x.floor(), z - z.floor());
    let s = |t: f64| t * t * t * (t * (t * 6.0 - 15.0) + 10.0);
    let (u, v) = (s(fx), s(fz));
    let w = |a: i64| period.map_or(a, |p| a.rem_euclid(p));
    let h = |a: i64, b: i64| hash(w(a), w(b), seed);
    let a = h(i, j) + (h(i + 1, j) - h(i, j)) * u;
    let b = h(i, j + 1) + (h(i + 1, j + 1) - h(i, j + 1)) * u;
    a + (b - a) * v
}

fn fbm(x: f64, z: f64, octaves: u32, seed: u64, period: Option<i64>) -> f64 {
    let (mut sum, mut amp, mut freq, mut norm) = (0.0, 1.0, 1.0, 0.0);
    for o in 0..octaves {
        sum += amp * noise(x * freq, z * freq, seed + u64::from(o), period.map(|p| p * freq as i64));
        norm += amp;
        amp *= 0.5;
        freq *= 2.0;
    }
    sum / norm
}

/// The world's height in meters at absolute (x, z): ridged mountains in
/// the middle, rolling valleys, a coast toward the edges.
pub fn world_height(x: f64, z: f64, size: f64) -> f64 {
    let (nx, nz) = (x / 2400.0, z / 2400.0);
    // Domain warp for less grid-like ridges.
    let wx = nx + 0.6 * (fbm(nx * 0.7, nz * 0.7, 3, 11, None) - 0.5);
    let wz = nz + 0.6 * (fbm(nx * 0.7 + 5.2, nz * 0.7 + 1.3, 3, 12, None) - 0.5);
    let mut ridge = 0.0;
    let (mut amp, mut freq) = (1.0, 1.0);
    for o in 0..7 {
        let n = 1.0 - (2.0 * noise(wx * freq, wz * freq, 40 + o, None) - 1.0).abs();
        ridge += amp * n * n;
        amp *= 0.47;
        freq *= 2.03;
    }
    let hills = fbm(nx * 2.5, nz * 2.5, 6, 70, None);
    // Mountain mask: high in the middle, low toward the coast.
    let c = size * 0.5;
    let r = (((x - c) / c).powi(2) + ((z - c) / c).powi(2)).sqrt();
    let land = (1.0 - r * 0.95).clamp(0.0, 1.0);
    let mask = fbm(nx * 0.4 + 3.0, nz * 0.4, 3, 90, None);
    let mountains = (mask * 1.6 - 0.45).clamp(0.0, 1.0) * land;
    let base = land * 120.0 + hills * 90.0 * land - 40.0 * (1.0 - land);
    base + mountains * ridge * 950.0
}

// ---------------------------------------------------------------- source

const SOURCE_QUADS: u32 = 64;
const SOURCE_SPACING: f32 = 4.0;
const TEXELS: u32 = 16;
/// Bump to invalidate the cached bake when the generator changes.
const GEN_VERSION: u32 = 3;
const LAYERS: [&str; 5] = ["grass", "rock", "dirt", "snow", "sand"];

struct Synthetic {
    tiles: i64,
    size: f64,
}

impl Synthetic {
    fn new(size_km: f64) -> Self {
        let edge = f64::from(SOURCE_QUADS) * f64::from(SOURCE_SPACING);
        let tiles = (size_km * 1000.0 / edge).ceil() as i64;
        Self { tiles, size: tiles as f64 * edge }
    }

    fn origin(tx: i64, tz: i64) -> DVec3 {
        let edge = f64::from(SOURCE_QUADS) * f64::from(SOURCE_SPACING);
        DVec3::new(tx as f64 * edge, 0.0, tz as f64 * edge)
    }

    fn fingerprint(id: u64) -> [u8; 32] {
        Digest::of(format!("synthetic/{GEN_VERSION}/{id}").as_bytes()).0
    }

    /// Layer index for a point, by height, slope and noise.
    fn layer_at(&self, x: f64, z: f64) -> u16 {
        let h = world_height(x, z, self.size);
        let e = 6.0;
        let dx = world_height(x + e, z, self.size) - world_height(x - e, z, self.size);
        let dz = world_height(x, z + e, self.size) - world_height(x, z - e, self.size);
        let slope = (dx * dx + dz * dz).sqrt() / (2.0 * e);
        let n = fbm(x / 90.0, z / 90.0, 3, 300, None);
        if h < 2.5 + 3.0 * n {
            4 // sand
        } else if h > 620.0 + 160.0 * n && slope < 0.9 {
            3 // snow
        } else if slope > 0.55 + 0.2 * n {
            1 // rock
        } else if n > 0.62 || slope > 0.35 {
            2 // dirt
        } else {
            0 // grass
        }
    }
}

impl TerrainSource for Synthetic {
    fn tiles(&self, _ws: &WorldspaceId, out: &mut dyn FnMut(SourceTileInfo)) {
        let edge = f32::from(u16::try_from(SOURCE_QUADS).unwrap()) * SOURCE_SPACING;
        for tz in 0..self.tiles {
            for tx in 0..self.tiles {
                let id = (tz * self.tiles + tx) as u64;
                let min = WorldPos::from_f64(Self::origin(tx, tz));
                out(SourceTileInfo {
                    id: SourceTileId(id),
                    fingerprint: Fingerprint(Self::fingerprint(id)),
                    min,
                    max: min.offset(Vec3::new(edge, 0.0, edge)),
                });
            }
        }
    }

    fn read(&self, _ws: &WorldspaceId, tile: SourceTileId) -> Option<SourceTile> {
        let id = tile.0 as i64;
        let (tx, tz) = (id % self.tiles, id / self.tiles);
        if tz >= self.tiles {
            return None;
        }
        let o = Self::origin(tx, tz);
        let n = SOURCE_QUADS + 1;
        let s = f64::from(SOURCE_SPACING);
        let mut heights = Vec::with_capacity((n * n) as usize);
        for j in 0..n {
            for i in 0..n {
                heights.push(world_height(o.x + f64::from(i) * s, o.z + f64::from(j) * s, self.size) as f32);
            }
        }
        let texel = s * f64::from(SOURCE_QUADS / TEXELS);
        let mut texels = Vec::with_capacity((TEXELS * TEXELS) as usize);
        for j in 0..TEXELS {
            for i in 0..TEXELS {
                texels.push(self.layer_at(o.x + (f64::from(i) + 0.5) * texel, o.z + (f64::from(j) + 0.5) * texel));
            }
        }
        let tn = TEXELS + 1;
        let rgb = (0..tn * tn)
            .map(|k| {
                let (x, z) = (o.x + f64::from(k % tn) * texel, o.z + f64::from(k / tn) * texel);
                let v = fbm(x / 700.0, z / 700.0, 3, 500, None);
                let w = fbm(x / 300.0, z / 300.0, 2, 501, None);
                [(205.0 + 50.0 * v) as u8, (215.0 + 40.0 * w) as u8, (210.0 + 45.0 * (1.0 - v)) as u8]
            })
            .collect();
        let origin = WorldPos::from_f64(o);
        Some(SourceTile {
            origin,
            spacing: SOURCE_SPACING,
            size: UVec2::splat(n),
            heights,
            surface: Triangulation::Fixed(Diagonal::Main),
            layers: Some(SourceLayers {
                origin,
                spacing: texel as f32,
                size: UVec2::splat(TEXELS),
                palette: LAYERS.iter().map(|l| RecordId::new(l)).collect(),
                texels,
            }),
            tint: Some(SourceTint { origin, spacing: texel as f32, size: UVec2::splat(tn), rgb }),
        })
    }
}

// ---------------------------------------------------------------- layers

const TEX: u32 = 1024;

/// Tileable procedural textures: albedo + height, normal, ARM.
fn layer_textures(name: &str) -> [Vec<u8>; 3] {
    let n = TEX as usize;
    let p = 16i64; // noise period in cells at frequency 1
    let mut height = vec![0.0f64; n * n];
    let mut albedo = vec![[0.0f64; 3]; n * n];
    let mut rough = vec![0.0f64; n * n];
    for y in 0..n {
        for x in 0..n {
            let (u, v) = (x as f64 / n as f64 * p as f64, y as f64 / n as f64 * p as f64);
            let f = |o: u32, s: u64, sc: f64| fbm(u * sc, v * sc, o, s, Some((p as f64 * sc) as i64));
            let (h, c, r) = match name {
                "grass" => {
                    let blades = f(5, 1, 4.0);
                    let patch = f(3, 2, 0.5);
                    let h = 0.35 + 0.5 * blades * blades;
                    let g = 0.6 + 0.4 * blades;
                    ([0.09 * g + 0.05 * patch, 0.16 * g + 0.04 * patch, 0.04 * g], h, 0.85).into_layer()
                }
                "rock" => {
                    let big = f(6, 10, 0.5);
                    let cracks = 1.0 - (2.0 * f(4, 11, 1.0) - 1.0).abs();
                    let h = 0.2 + 0.65 * big - 0.25 * cracks.powi(8);
                    let g = 0.22 + 0.25 * big - 0.12 * cracks.powi(8);
                    ([g * 1.02, g * 0.98, g * 0.93], h, 0.62 + 0.2 * f(2, 12, 2.0)).into_layer()
                }
                "dirt" => {
                    let soil = f(5, 20, 1.0);
                    let pebble = (f(2, 21, 8.0) - 0.6).max(0.0) * 3.0;
                    let h = 0.3 + 0.3 * soil + 0.5 * pebble;
                    (
                        [
                            0.19 + 0.08 * soil + 0.1 * pebble,
                            0.13 + 0.06 * soil + 0.09 * pebble,
                            0.08 + 0.05 * soil + 0.08 * pebble,
                        ],
                        h,
                        0.9 - 0.2 * pebble,
                    )
                        .into_layer()
                }
                "snow" => {
                    let drift = f(4, 30, 0.5);
                    let h = 0.5 + 0.2 * drift;
                    ([0.8 + 0.08 * drift, 0.83 + 0.08 * drift, 0.88 + 0.06 * drift], h, 0.45 + 0.2 * f(3, 31, 4.0))
                        .into_layer()
                }
                _ => {
                    let ripple = (u * std::f64::consts::TAU * 2.0 + 3.0 * f(3, 41, 0.5)).sin() * 0.5 + 0.5;
                    let grain = f(2, 40, 8.0);
                    let h = 0.35 + 0.35 * ripple + 0.1 * grain;
                    ([0.55 + 0.08 * grain, 0.47 + 0.07 * grain, 0.33 + 0.05 * grain], h, 0.8).into_layer()
                }
            };
            let k = y * n + x;
            height[k] = h;
            albedo[k] = c;
            rough[k] = r;
        }
    }
    let srgb = |v: f64| {
        let v = v.clamp(0.0, 1.0);
        let s = if v <= 0.003_130_8 { v * 12.92 } else { 1.055 * v.powf(1.0 / 2.4) - 0.055 };
        (s * 255.0).round() as u8
    };
    let unorm = |v: f64| (v.clamp(0.0, 1.0) * 255.0).round() as u8;
    let mut ah = header(TEX);
    let mut nm = header(TEX);
    let mut arm = header(TEX);
    let strength = if name == "rock" { 6.0 } else { 3.0 };
    for y in 0..n {
        for x in 0..n {
            let k = y * n + x;
            let c = albedo[k];
            ah.extend_from_slice(&[srgb(c[0]), srgb(c[1]), srgb(c[2]), unorm(height[k])]);
            let hx = height[y * n + (x + 1) % n] - height[y * n + (x + n - 1) % n];
            let hy = height[((y + 1) % n) * n + x] - height[((y + n - 1) % n) * n + x];
            let v = Vec3::new(-(hx * strength) as f32, -(hy * strength) as f32, 1.0).normalize();
            nm.extend_from_slice(&[
                unorm(f64::from(v.x) * 0.5 + 0.5),
                unorm(f64::from(v.y) * 0.5 + 0.5),
                unorm(f64::from(v.z) * 0.5 + 0.5),
                255,
            ]);
            let ao = 0.55 + 0.45 * height[k].clamp(0.0, 1.0).sqrt();
            arm.extend_from_slice(&[unorm(ao), unorm(rough[k]), 0, 255]);
        }
    }
    [ah, nm, arm]
}

trait IntoLayer {
    fn into_layer(self) -> (f64, [f64; 3], f64);
}

impl IntoLayer for ([f64; 3], f64, f64) {
    fn into_layer(self) -> (f64, [f64; 3], f64) {
        (self.1, self.0, self.2)
    }
}

fn header(size: u32) -> Vec<u8> {
    let mut v = Vec::with_capacity(8 + (size * size * 4) as usize);
    v.extend_from_slice(&size.to_le_bytes());
    v.extend_from_slice(&size.to_le_bytes());
    v
}

struct Layers;

impl LayerSource for Layers {
    fn layers(&self) -> Vec<RecordId> {
        let mut ids: Vec<RecordId> = LAYERS.iter().map(|l| RecordId::new(l)).collect();
        ids.sort();
        ids
    }
    fn layer(&self, id: &RecordId) -> Option<TerrainLayer> {
        let name = id.as_str();
        LAYERS.contains(&name).then(|| TerrainLayer {
            albedo_height: format!("terrain/{name}_ah.raw"),
            normal: Some(format!("terrain/{name}_n.raw")),
            arm: Some(format!("terrain/{name}_arm.raw")),
            tiling_m: match name {
                "rock" => 9.0,
                "snow" => 7.0,
                _ => 4.0,
            },
            triplanar: name == "rock",
            displacement_m: match name {
                "rock" => 0.08,
                "dirt" => 0.05,
                _ => 0.03,
            },
            surface: None,
        })
    }
    fn decode(&self, _path: &str, bytes: &[u8]) -> Result<LayerImage, String> {
        let w = u32::from_le_bytes(bytes[0..4].try_into().map_err(|_| "short")?);
        let h = u32::from_le_bytes(bytes[4..8].try_into().map_err(|_| "short")?);
        Ok(LayerImage { width: w, height: h, rgba: bytes[8..].to_vec() })
    }
}

/// The bake host: fingerprints and files. Textures are generated on first use.
struct Host {
    files: std::sync::Mutex<HashMap<String, Arc<Vec<u8>>>>,
}

impl Host {
    fn file(&self, path: &str) -> Option<Arc<Vec<u8>>> {
        let name = path.strip_prefix("terrain/")?.strip_suffix(".raw")?;
        let (layer, kind) = name.rsplit_once('_')?;
        if !LAYERS.contains(&layer) {
            return None;
        }
        let mut files = self.files.lock().unwrap();
        if !files.contains_key(path) {
            let [ah, n, arm] = layer_textures(layer);
            files.insert(format!("terrain/{layer}_ah.raw"), Arc::new(ah));
            files.insert(format!("terrain/{layer}_n.raw"), Arc::new(n));
            files.insert(format!("terrain/{layer}_arm.raw"), Arc::new(arm));
        }
        let _ = kind;
        files.get(path).cloned()
    }
}

impl Inputs for Host {
    fn file_fingerprint(&self, path: &str) -> Result<Option<Digest>, BakeError> {
        // Generated content: the generator version and path identify it.
        Ok(self.file(path).map(|_| Digest::of(format!("{GEN_VERSION}/{TEX}/{path}").as_bytes())))
    }
    fn read_file(&self, path: &str) -> Result<Vec<u8>, BakeError> {
        self.file(path).map(|b| b.to_vec()).ok_or_else(|| BakeError::Other(format!("no file {path}")))
    }
    fn record_fingerprint(&self, kind: &str, id: &str) -> Result<Option<Digest>, BakeError> {
        Ok(match kind {
            SOURCE_TILE_KIND => {
                let n: u64 = id.rsplit('|').next().and_then(|s| s.parse().ok()).unwrap_or(u64::MAX);
                Some(Digest(Synthetic::fingerprint(n)))
            }
            LAYER_KIND => Some(Digest::of(format!("{GEN_VERSION}/layer/{id}").as_bytes())),
            _ => None,
        })
    }
}

pub fn bake(size_km: f64, rebake: bool) -> (Arc<BakedTerrain>, f64) {
    let source = Arc::new(Synthetic::new(size_km));
    let dir = std::env::temp_dir().join(format!("sf_terrain_example_{}km", size_km as u32));
    if rebake {
        let _ = std::fs::remove_dir_all(&dir);
    }
    let store = Store::open(&dir).expect("bake store");
    let ws = WorldspaceId::new("synthetic");
    let config = TerrainBakeConfig { fill_height: -60.0, default_layer: "sand".into(), ..TerrainBakeConfig::default() };
    let bake = TerrainBake::new(source.clone(), Arc::new(NoFootprints), Arc::new(Layers), config);
    let mut baker = Baker::new(store.clone(), Arc::new(Host { files: std::sync::Mutex::new(HashMap::new()) }));
    bake.register(&mut baker);
    let t = Instant::now();
    let keys = bake.products(&ws, TEX);
    let failed = bake.bake(&baker, &ws, TEX);
    for (k, e) in failed.iter().take(5) {
        eprintln!("bake failed: {k}: {e}");
    }
    println!(
        "terrain bake: {} products in {:.2} s ({} source tiles, {:.1} km)",
        keys.len(),
        t.elapsed().as_secs_f64(),
        source.tiles * source.tiles,
        source.size / 1000.0
    );
    for (step, s) in &baker.step_stats() {
        println!(
            "  {step:20} built {:5} hit {:5} run {:8.2} s (summed over threads) {:8.1} MiB",
            s.built,
            s.hits,
            s.run_time.as_secs_f64(),
            s.bytes_written as f64 / 1_048_576.0
        );
    }
    let manifest = baker.manifest();
    store.save_manifest("example", &manifest).expect("manifest");
    (Arc::new(BakedTerrain::new(store, &manifest, ws)), source.size)
}
