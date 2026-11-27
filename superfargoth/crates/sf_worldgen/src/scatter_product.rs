//! The `scatter/` guide product (worldgen-scatter.md "Product", slice WG-E2j): the kept set of a scatter v2 run
//! ([`LayerWinners`]: every layer's winners, before the budget cut) written once to the guide folder, keyed by a
//! blake3 of the run's inputs, and read back at load. The budget cut and the pose stay at load, so
//! `--set scatter.budget=N` never rebuilds the product.
//!
//! **Inputs hash.** [`InputsHash`] feeds blake3 with named, length-prefixed parts, under a domain tag with
//! [`FORMAT`]: the guide ([`InputsHash::guide`]: dims, spacing, origin, sea level, heights, regions), the climate
//! product ([`InputsHash::climate`]: dims, spacing, origin, class names, [`ClimateMap::hash`]), the seed, named
//! values and any adapter bytes (species file text, learned model rows, clearings). The adapter names what it feeds;
//! a part left out never invalidates the product, so every input of the passes must be fed.
//!
//! Files (little-endian, fixed-size records in output order): `plants.bin` (64 bytes each: layer u32, species u32,
//! tile i32 x2, parent u32, offspring u32, pos f64 x2, size f64, crown f64, tiebreak u64) and `rocks.bin` (72 bytes:
//! context u32, pad u32, tile i32 x2, parent u32, offspring u32, pos f64 x2, size f64, tiebreak u64, source f64 x2);
//! `meta.json` holds the format, the inputs hash, the record counts, per-file and product blake3. Floats are kept
//! as f64 so a read equals the computed set bit for bit.

use std::path::Path;

use crate::climate_map::ClimateMap;
use crate::guide::Guide;
use crate::scatter::{Candidate, CandidateKey, LayerWinners};
use crate::scatter_fields::GridFields;
use crate::scatter_rocks::{RockCandidate, RockKey};
use crate::scatter_species::{Layer, RockContext};
use crate::{Error, Result};

#[cfg(test)]
#[path = "tests_scatter_product.rs"]
mod tests;

/// Product format version (part of the inputs hash: a new format rebuilds). 2: WG-E2o.f sampled calibration (same
/// inputs, other `g`).
// tunables: const (format)
pub const FORMAT: u32 = 2;

/// The product's folder inside the guide folder.
// tunables: const (format)
pub const DIR: &str = "scatter";

/// Bytes of one `plants.bin` record.
// tunables: const (format)
pub const PLANT_BYTES: usize = 64;

/// Bytes of one `rocks.bin` record.
// tunables: const (format)
pub const ROCK_BYTES: usize = 72;

/// blake3 of a scatter run's inputs (module docs).
#[derive(Clone)]
pub struct InputsHash {
    hasher: blake3::Hasher,
}

impl Default for InputsHash {
    fn default() -> Self {
        Self::new()
    }
}

impl InputsHash {
    /// An empty hash under the product's domain tag and [`FORMAT`].
    #[must_use]
    pub fn new() -> Self {
        let mut hasher = blake3::Hasher::new();
        hasher.update(b"sf_worldgen scatter product inputs");
        hasher.update(&FORMAT.to_le_bytes());
        Self { hasher }
    }

    /// Feeds one named part (name and bytes, each length-prefixed, so parts never run into each other).
    pub fn bytes(&mut self, name: &str, bytes: &[u8]) -> &mut Self {
        for b in [name.as_bytes(), bytes] {
            self.hasher.update(&(b.len() as u64).to_le_bytes());
            self.hasher.update(b);
        }
        self
    }

    /// Feeds a named f64 value (its bits: `0.0` and `-0.0` differ, as they may in the passes).
    pub fn value(&mut self, name: &str, v: f64) -> &mut Self {
        self.bytes(name, &v.to_bits().to_le_bytes())
    }

    /// Feeds the run's seed.
    pub fn seed(&mut self, seed: u64) -> &mut Self {
        self.bytes("seed", &seed.to_le_bytes())
    }

    /// Feeds the guide: dims, spacing, origin, sea level, fill, heights and the region grid with its legend.
    pub fn guide(&mut self, g: &Guide) -> &mut Self {
        let mut head = Vec::with_capacity(64);
        for v in [g.w as u64, g.h as u64] {
            head.extend_from_slice(&v.to_le_bytes());
        }
        for v in [g.spacing_m, g.origin_m[0], g.origin_m[1], g.sea_level_m, g.fill_m.unwrap_or(f64::NAN)] {
            head.extend_from_slice(&v.to_bits().to_le_bytes());
        }
        self.bytes("guide.head", &head);
        let heights: Vec<u8> = g.heights.iter().flat_map(|h| h.to_bits().to_le_bytes()).collect();
        self.bytes("guide.heights", &heights);
        if let Some(r) = &g.regions {
            let mut head = Vec::with_capacity(24);
            for v in [r.w as u64, r.h as u64, r.cell_m.to_bits()] {
                head.extend_from_slice(&v.to_le_bytes());
            }
            self.bytes("guide.regions.head", &head);
            self.bytes("guide.regions.index", &r.index);
            self.bytes("guide.regions.names", r.names.join("\n").as_bytes());
        } else {
            self.bytes("guide.regions.head", &[]);
        }
        self
    }

    /// Feeds the climate product: dims, spacing, origin, class names and its product hash.
    pub fn climate(&mut self, c: &ClimateMap) -> &mut Self {
        let mut head = Vec::with_capacity(40);
        for v in [c.w as u64, c.h as u64, c.spacing_m.to_bits(), c.origin_m[0].to_bits(), c.origin_m[1].to_bits()] {
            head.extend_from_slice(&v.to_le_bytes());
        }
        self.bytes("climate.head", &head);
        self.bytes("climate.classes", c.class_names.join("\n").as_bytes());
        self.bytes("climate.blake3", c.hash().as_bytes())
    }

    /// Feeds product [`GridFields`] (what the passes sample): dims, spacing, origin, class names and every raster.
    /// Finer than [`InputsHash::climate`] (slope, aspect, curvature and the water distance too); an adapter feeds one.
    pub fn fields(&mut self, f: &GridFields) -> &mut Self {
        let mut head = Vec::with_capacity(40);
        for v in [f.w as u64, f.h as u64, f.spacing_m.to_bits(), f.origin_m[0].to_bits(), f.origin_m[1].to_bits()] {
            head.extend_from_slice(&v.to_le_bytes());
        }
        self.bytes("fields.head", &head);
        self.bytes("fields.classes.names", f.class_names.join("
").as_bytes());
        self.bytes("fields.classes", &f.classes);
        let rasters = [
            ("fields.temperature_c", &f.temperature_c),
            ("fields.precipitation_mm", &f.precipitation_mm),
            ("fields.moisture", &f.moisture),
            ("fields.slope_deg", &f.slope_deg),
            ("fields.aspect_deg", &f.aspect_deg),
            ("fields.altitude_m", &f.altitude_m),
            ("fields.curvature", &f.curvature),
            ("fields.water_distance_m", &f.water_distance_m),
        ];
        for (name, v) in rasters {
            let bytes: Vec<u8> = v.iter().flat_map(|x| x.to_bits().to_le_bytes()).collect();
            self.bytes(name, &bytes);
        }
        self
    }

    /// The hash, hex.
    #[must_use]
    pub fn finish(&self) -> String {
        self.hasher.finalize().to_hex().to_string()
    }
}

fn layer_id(l: Layer) -> u32 {
    match l {
        Layer::Canopy => 0,
        Layer::Understorey => 1,
        Layer::Ground => 2,
    }
}

fn layer_of(id: u32) -> Option<Layer> {
    [Layer::Canopy, Layer::Understorey, Layer::Ground].get(id as usize).copied()
}

fn context_id(c: RockContext) -> u32 {
    match c {
        RockContext::Talus => 0,
        RockContext::Outcrop => 1,
        RockContext::Field => 2,
    }
}

fn context_of(id: u32) -> Option<RockContext> {
    [RockContext::Talus, RockContext::Outcrop, RockContext::Field].get(id as usize).copied()
}

fn plant_bytes(plants: &[Candidate]) -> Vec<u8> {
    let mut out = Vec::with_capacity(plants.len() * PLANT_BYTES);
    for c in plants {
        let k = &c.key;
        for v in [layer_id(k.layer), k.species] {
            out.extend_from_slice(&v.to_le_bytes());
        }
        for v in [k.tile.0, k.tile.1] {
            out.extend_from_slice(&v.to_le_bytes());
        }
        for v in [k.parent, k.offspring] {
            out.extend_from_slice(&v.to_le_bytes());
        }
        for v in [c.pos[0], c.pos[1], c.size_m, c.crown_m] {
            out.extend_from_slice(&v.to_le_bytes());
        }
        out.extend_from_slice(&c.tiebreak.to_le_bytes());
    }
    out
}

fn rock_bytes(rocks: &[RockCandidate]) -> Vec<u8> {
    let mut out = Vec::with_capacity(rocks.len() * ROCK_BYTES);
    for r in rocks {
        let k = &r.key;
        for v in [context_id(k.context), 0] {
            out.extend_from_slice(&v.to_le_bytes());
        }
        for v in [k.tile.0, k.tile.1] {
            out.extend_from_slice(&v.to_le_bytes());
        }
        for v in [k.parent, k.offspring] {
            out.extend_from_slice(&v.to_le_bytes());
        }
        for v in [r.pos[0], r.pos[1], r.size_m] {
            out.extend_from_slice(&v.to_le_bytes());
        }
        out.extend_from_slice(&r.tiebreak.to_le_bytes());
        for v in r.source {
            out.extend_from_slice(&v.to_le_bytes());
        }
    }
    out
}

/// Little-endian field reader over one record.
struct Rec<'a>(&'a [u8]);

impl Rec<'_> {
    fn take<const N: usize>(&mut self) -> [u8; N] {
        let (head, rest) = self.0.split_at(N);
        self.0 = rest;
        head.try_into().unwrap_or([0; N])
    }
    fn u32(&mut self) -> u32 {
        u32::from_le_bytes(self.take())
    }
    fn i32(&mut self) -> i32 {
        i32::from_le_bytes(self.take())
    }
    fn u64(&mut self) -> u64 {
        u64::from_le_bytes(self.take())
    }
    fn f64(&mut self) -> f64 {
        f64::from_le_bytes(self.take())
    }
}

fn plants_of(bytes: &[u8]) -> std::result::Result<Vec<Candidate>, String> {
    bytes
        .as_chunks::<PLANT_BYTES>()
        .0
        .iter()
        .enumerate()
        .map(|(i, b)| {
            let mut r = Rec(b);
            let id = r.u32();
            let layer = layer_of(id).ok_or_else(|| format!("plant {i} has layer id {id}"))?;
            let species = r.u32();
            let tile = (r.i32(), r.i32());
            let (parent, offspring) = (r.u32(), r.u32());
            let pos = [r.f64(), r.f64()];
            let (size_m, crown_m) = (r.f64(), r.f64());
            let tiebreak = r.u64();
            Ok(Candidate { key: CandidateKey { layer, species, tile, parent, offspring }, pos, size_m, crown_m, tiebreak })
        })
        .collect()
}

fn rocks_of(bytes: &[u8]) -> std::result::Result<Vec<RockCandidate>, String> {
    bytes
        .as_chunks::<ROCK_BYTES>()
        .0
        .iter()
        .enumerate()
        .map(|(i, b)| {
            let mut r = Rec(b);
            let id = r.u32();
            let context = context_of(id).ok_or_else(|| format!("rock {i} has context id {id}"))?;
            let _pad = r.u32();
            let tile = (r.i32(), r.i32());
            let (parent, offspring) = (r.u32(), r.u32());
            let pos = [r.f64(), r.f64()];
            let size_m = r.f64();
            let tiebreak = r.u64();
            let source = [r.f64(), r.f64()];
            Ok(RockCandidate { key: RockKey { context, tile, parent, offspring }, pos, size_m, tiebreak, source })
        })
        .collect()
}

/// `scatter/meta.json`.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct MetaJson {
    format: u32,
    producer: String,
    inputs_blake3: String,
    plants: usize,
    rocks: usize,
    files: std::collections::BTreeMap<String, String>,
    blake3: String,
}

/// Writes `dir/scatter/` (module docs) for the run of inputs hash `inputs`; returns the product blake3.
pub fn write(dir: &Path, inputs: &str, won: &LayerWinners) -> Result<String> {
    let out = dir.join(DIR);
    std::fs::create_dir_all(&out)?;
    let layers = [("plants.bin", plant_bytes(&won.plants)), ("rocks.bin", rock_bytes(&won.rocks))];
    let mut files = std::collections::BTreeMap::new();
    let mut total = blake3::Hasher::new();
    for (name, b) in &layers {
        std::fs::write(out.join(name), b)?;
        total.update(b);
        files.insert((*name).to_owned(), blake3::hash(b).to_hex().to_string());
    }
    let hash = total.finalize().to_hex().to_string();
    let meta = MetaJson {
        format: FORMAT,
        producer: "sf_worldgen scatter v2 (WG-E2j)".into(),
        inputs_blake3: inputs.to_owned(),
        plants: won.plants.len(),
        rocks: won.rocks.len(),
        files,
        blake3: hash.clone(),
    };
    std::fs::write(out.join("meta.json"), serde_json::to_string_pretty(&meta)?)?;
    Ok(hash)
}

/// Reads `dir/scatter/` (written by [`write`]): the format, every file's size and blake3 and the product blake3 are
/// checked against `meta.json`, and every layer and context id. Returns the inputs hash it was built from and the
/// kept set.
pub fn read(dir: &Path) -> Result<(String, LayerWinners)> {
    let src = dir.join(DIR);
    let bad = |what: String| Error::Invalid(format!("scatter product {}: {what}", src.display()));
    let meta: MetaJson = serde_json::from_str(&std::fs::read_to_string(src.join("meta.json"))?)?;
    if meta.format != FORMAT {
        return Err(bad(format!("format {}, this build reads {FORMAT}", meta.format)));
    }
    let names = [("plants.bin", PLANT_BYTES, meta.plants), ("rocks.bin", ROCK_BYTES, meta.rocks)];
    if let Some(extra) = meta.files.keys().find(|k| !names.iter().any(|(name, _, _)| name == k)) {
        return Err(bad(format!("meta.json lists an unknown file {extra}")));
    }
    let mut total = blake3::Hasher::new();
    let mut layers = Vec::with_capacity(names.len());
    for (name, record, count) in names {
        let want = meta.files.get(name).ok_or_else(|| bad(format!("meta.json has no blake3 for {name}")))?;
        let bytes = std::fs::read(src.join(name))?;
        if bytes.len() != record * count {
            return Err(bad(format!("{name} is {} bytes, {count} records need {}", bytes.len(), record * count)));
        }
        let got = blake3::hash(&bytes).to_hex().to_string();
        if &got != want {
            return Err(bad(format!("{name} blake3 {got} differs from meta.json's {want}")));
        }
        total.update(&bytes);
        layers.push(bytes);
    }
    let got = total.finalize().to_hex().to_string();
    if got != meta.blake3 {
        return Err(bad(format!("product blake3 {got} differs from meta.json's {}", meta.blake3)));
    }
    let plants = plants_of(&layers[0]).map_err(bad)?;
    let rocks = rocks_of(&layers[1]).map_err(bad)?;
    Ok((meta.inputs_blake3, LayerWinners { rocks, plants }))
}

/// The kept set of `dir/scatter/` when it was built from `inputs`; `Ok(None)` when there is no product, or it is
/// of another format or other inputs (stale: the caller rebuilds and [`write`]s). A product of these inputs that
/// fails its checks is an error (the caller logs it and rebuilds).
pub fn cached(dir: &Path, inputs: &str) -> Result<Option<LayerWinners>> {
    let meta_path = dir.join(DIR).join("meta.json");
    let Ok(text) = std::fs::read_to_string(&meta_path) else { return Ok(None) };
    let Ok(meta) = serde_json::from_str::<MetaJson>(&text) else { return Ok(None) };
    if meta.format != FORMAT || meta.inputs_blake3 != inputs {
        return Ok(None);
    }
    read(dir).map(|(_, won)| Some(won))
}
