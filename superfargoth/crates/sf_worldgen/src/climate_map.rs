//! The `climate/` guide product: temperature, precipitation, moisture and climate class per guide sample.
//!
//! [`build`] runs, in order: heights above sea level (`height - sea_level_m`) -> [`temperature`] ->
//! [`precipitation`] (the sweep's water [`Budget`] is kept) -> the params file's region overrides, resolved by
//! legend name against the guide's region grid ([`apply_overrides`], which floors rain at 0) -> the drainage area,
//! computed here by MFD ([`mfd_area`]) on the Priority-Flood+ε surface of the heights above sea level (sea =
//! `<= 0`) -> [`moisture`] and [`classes`] on the overridden fields.
//!
//! Files (row-major, row 0 north, little-endian, one value per guide sample): `temperature.f32` (C),
//! `precipitation.f32` (mm/yr, never below 0), `moisture.f32` (0..1), `classes.u8` (index into
//! `climate.json` `classes`); plus `climate.json` (class names, declared values, applied overrides) and
//! `meta.json` (dims, spacing, origin, per-file and product blake3). [`ClimateMap::read`] loads it back, every file's
//! blake3 checked against `meta.json` (a product without the `budget` / `mfd_stuck` diagnostics of `climate.json`,
//! written before 2026-11-27, reads them as 0).
//!
//! [`run_climate`] is one `scrib-worldgen climate` invocation: guide folder in, `climate/` out, one summary line.

use std::path::{Path, PathBuf};
use std::time::Instant;

use rayon::prelude::*;

use crate::climate::{moisture, precipitation, temperature, Budget, Climate, ClimateFile};
use crate::climate_classes::{classes, ClassTable};
use crate::climate_override::{apply_overrides, Adjust, RegionOverride};
use crate::filter::Declared;
use crate::flood::priority_flood_eps;
use crate::grid::Grid;
use crate::guide::Guide;
use crate::mfd::mfd_area;
use crate::{Error, Result};

#[cfg(test)]
#[path = "tests_climate_map.rs"]
mod tests;

/// Product format version.
pub const FORMAT: u32 = 1;

type Layer = (&'static str, Vec<u8>);

/// One override as applied: the legend name and the index it resolved to.
#[derive(Debug, Clone, PartialEq)]
pub struct AppliedOverride {
    pub name: String,
    pub index: u8,
    pub temperature: Adjust,
    pub rain: Adjust,
}

/// The climate rasters of one guide (one value per guide sample, row-major, row 0 north).
#[derive(Debug, Clone)]
pub struct ClimateMap {
    pub w: usize,
    pub h: usize,
    /// Sample spacing, metres.
    pub spacing_m: f64,
    /// Guide-frame metres of sample (0, 0).
    pub origin_m: [f64; 2],
    /// C, after the overrides.
    pub temperature: Vec<f32>,
    /// mm/yr, after the overrides, never below 0.
    pub precipitation: Vec<f32>,
    /// 0..1.
    pub moisture: Vec<f32>,
    /// Index into `class_names`.
    pub classes: Vec<u8>,
    pub class_names: Vec<String>,
    pub overrides: Vec<AppliedOverride>,
    /// Water budget of the precipitation sweep, before the overrides.
    pub budget: Budget,
    /// MFD senders with no lower neighbour (0 on a Priority-Flood+ε surface).
    pub mfd_stuck: usize,
}

/// Wall time of one build stage: name, seconds.
pub type StageTime = (&'static str, f64);

/// Sea mask (`<= 0`) and Priority-Flood+ε routing surface of heights above sea level.
fn routing_surface(heights: &Grid) -> (Vec<bool>, Vec<f64>) {
    let wet: Vec<bool> = heights.data.iter().map(|&x| x <= 0.0).collect();
    let mut z = Vec::new();
    priority_flood_eps(heights.w, heights.h, &heights.data, &wet, 0.0, &mut z);
    (wet, z)
}

/// Builds the climate product of `guide` (module docs for the order).
pub fn build(guide: &Guide, file: &ClimateFile, table: &ClassTable) -> Result<ClimateMap> {
    build_timed(guide, file, table).map(|(map, _)| map)
}

/// [`build`], with the wall time of each stage in build order.
pub fn build_timed(guide: &Guide, file: &ClimateFile, table: &ClassTable) -> Result<(ClimateMap, Vec<StageTime>)> {
    let mut times: Vec<StageTime> = Vec::with_capacity(9);
    let mut clock = Instant::now();
    let mut lap = |name: &'static str, times: &mut Vec<StageTime>| {
        times.push((name, clock.elapsed().as_secs_f64()));
        clock = Instant::now();
    };
    let c = &file.climate;
    let (w, h) = (guide.w, guide.h);
    let heights = Grid {
        w,
        h,
        spacing: guide.spacing_m,
        data: guide.heights.iter().map(|&x| f64::from(x) - guide.sea_level_m).collect(),
    };
    lap("heights", &mut times);
    let t = temperature(&heights, c);
    lap("temperature", &mut times);
    let rain = precipitation(&heights, &t, c)?;
    lap("rain", &mut times);

    let mut applied = Vec::with_capacity(file.overrides.len());
    let (t, p) = if file.overrides.is_empty() {
        (t, rain.p)
    } else {
        let regions = guide.regions.as_ref().ok_or_else(|| {
            Error::Invalid("climate: the params file overrides regions but the guide has no region grid".into())
        })?;
        let mut list = Vec::with_capacity(file.overrides.len());
        for o in &file.overrides {
            let index = regions.find(&o.region).ok_or_else(|| {
                Error::Invalid(format!("climate: override region \"{}\" is not in the guide's legend", o.region))
            })?;
            list.push(RegionOverride { region: index, temperature: o.temperature, rain: o.rain });
            applied.push(AppliedOverride {
                name: o.region.clone(),
                index,
                temperature: o.temperature,
                rain: o.rain,
            });
        }
        let mut labels = vec![0u8; w * h];
        labels.par_chunks_mut(w).enumerate().for_each(|(row, line)| {
            for (col, l) in line.iter_mut().enumerate() {
                *l = regions.at_sample(col, row, guide.spacing_m);
            }
        });
        apply_overrides(&t, &rain.p, &labels, &list, c.override_blend_m)?
    };

    lap("overrides", &mut times);

    let (wet, z) = routing_surface(&heights);
    lap("flood", &mut times);
    let (area, mfd_stuck) = mfd_area(w, h, heights.spacing, &z, &wet, c.drain_flow_exponent);
    lap("mfd", &mut times);

    let m = moisture(&t, &p, &area, c)?;
    lap("moisture", &mut times);
    let cls = classes(&t, &p, table)?;
    lap("classes", &mut times);
    let to_f32 = |g: &Grid| g.data.iter().map(|&x| x as f32).collect::<Vec<f32>>();
    let map = ClimateMap {
        w,
        h,
        spacing_m: guide.spacing_m,
        origin_m: guide.origin_m,
        temperature: to_f32(&t),
        precipitation: to_f32(&p),
        moisture: to_f32(&m),
        classes: cls,
        class_names: table.names().to_vec(),
        overrides: applied,
        budget: rain.budget,
        mfd_stuck,
    };
    lap("to f32", &mut times);
    Ok((map, times))
}

fn adjust_json(a: &Adjust) -> serde_json::Value {
    serde_json::json!({ "scale": a.scale, "add": a.add, "pin": a.pin })
}

impl ClimateMap {
    /// The product files in their fixed order.
    fn layers(&self) -> Vec<Layer> {
        let bytes32 = |v: &[f32]| v.iter().flat_map(|x| x.to_le_bytes()).collect::<Vec<u8>>();
        vec![
            ("temperature.f32", bytes32(&self.temperature)),
            ("precipitation.f32", bytes32(&self.precipitation)),
            ("moisture.f32", bytes32(&self.moisture)),
            ("classes.u8", self.classes.clone()),
        ]
    }

    /// blake3 over every product file in order (the map's product hash).
    pub fn hash(&self) -> String {
        let mut hasher = blake3::Hasher::new();
        for (_, b) in self.layers() {
            hasher.update(&b);
        }
        hasher.finalize().to_hex().to_string()
    }

    /// Writes `dir/climate/` (module docs) and returns the product hash.
    pub fn write(&self, dir: &Path, declared: &[Declared]) -> Result<String> {
        let out = dir.join("climate");
        std::fs::create_dir_all(&out)?;
        let mut files = serde_json::Map::new();
        let mut total = blake3::Hasher::new();
        for (name, b) in self.layers() {
            std::fs::write(out.join(name), &b)?;
            total.update(&b);
            files.insert(name.into(), blake3::hash(&b).to_hex().to_string().into());
        }
        let hash = total.finalize().to_hex().to_string();
        let values: serde_json::Map<String, serde_json::Value> = declared
            .iter()
            .map(|d| {
                (
                    d.name.to_string(),
                    serde_json::json!({ "v": d.v, "unit": d.unit, "source": d.source }),
                )
            })
            .collect();
        let overrides: Vec<serde_json::Value> = self
            .overrides
            .iter()
            .map(|o| {
                serde_json::json!({
                    "region": o.name,
                    "index": o.index,
                    "temperature": adjust_json(&o.temperature),
                    "rain": adjust_json(&o.rain),
                })
            })
            .collect();
        std::fs::write(
            out.join("climate.json"),
            serde_json::to_string_pretty(&serde_json::json!({
                "format": FORMAT,
                "classes": self.class_names,
                "values": values,
                "overrides": overrides,
                "budget": {
                    "inflow": self.budget.inflow,
                    "source": self.budget.source,
                    "rain": self.budget.rain,
                    "outflow": self.budget.outflow,
                },
                "mfd_stuck": self.mfd_stuck,
            }))?,
        )?;
        std::fs::write(
            out.join("meta.json"),
            serde_json::to_string_pretty(&serde_json::json!({
                "format": FORMAT,
                "producer": "sf_worldgen climate (WG-E1)",
                "dims": [self.w, self.h],
                "sample_spacing_m": self.spacing_m,
                "origin_m": self.origin_m,
                "files": files,
                "blake3": hash,
            }))?,
        )?;
        Ok(hash)
    }

    /// Reads `dir/climate/` (written by [`ClimateMap::write`]): the format, every file's size and blake3 and the
    /// product blake3 are checked against `meta.json`, the class ids against `climate.json`'s class names.
    pub fn read(dir: &Path) -> Result<Self> {
        let src = dir.join("climate");
        let bad = |what: String| Error::Invalid(format!("climate product {}: {what}", src.display()));
        let meta: MetaJson = serde_json::from_str(&std::fs::read_to_string(src.join("meta.json"))?)?;
        let info: ClimateJson = serde_json::from_str(&std::fs::read_to_string(src.join("climate.json"))?)?;
        if meta.format != FORMAT || info.format != FORMAT {
            return Err(bad(format!(
                "format {} / {} (meta / climate.json), this build reads {FORMAT}",
                meta.format, info.format
            )));
        }
        let [w, h] = meta.dims;
        let n = w * h;
        // File, bytes per sample (the order of `layers`).
        let names = [("temperature.f32", 4), ("precipitation.f32", 4), ("moisture.f32", 4), ("classes.u8", 1)];
        if let Some(extra) = meta.files.keys().find(|k| !names.iter().any(|(name, _)| name == k)) {
            return Err(bad(format!("meta.json lists an unknown file {extra}")));
        }
        let mut total = blake3::Hasher::new();
        let mut layers = Vec::with_capacity(names.len());
        for (name, bytes_per_sample) in names {
            let want = meta.files.get(name).ok_or_else(|| bad(format!("meta.json has no blake3 for {name}")))?;
            let bytes = std::fs::read(src.join(name))?;
            let size = bytes_per_sample * n;
            if bytes.len() != size {
                return Err(bad(format!("{name} is {} bytes, {w} x {h} needs {size}", bytes.len())));
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
        if info.classes.len() > usize::from(u8::MAX) {
            return Err(bad(format!("{} class names, at most {}", info.classes.len(), u8::MAX)));
        }
        let classes = layers.pop().unwrap_or_default();
        if let Some(&c) = classes.iter().find(|&&c| usize::from(c) >= info.classes.len()) {
            return Err(bad(format!("class id {c} with {} class names", info.classes.len())));
        }
        let floats = |b: &[u8]| b.as_chunks::<4>().0.iter().map(|c| f32::from_le_bytes(*c)).collect::<Vec<f32>>();
        let budget = info.budget.unwrap_or(BudgetJson { inflow: 0.0, source: 0.0, rain: 0.0, outflow: 0.0 });
        Ok(Self {
            w,
            h,
            spacing_m: meta.sample_spacing_m,
            origin_m: meta.origin_m,
            temperature: floats(&layers[0]),
            precipitation: floats(&layers[1]),
            moisture: floats(&layers[2]),
            classes,
            class_names: info.classes,
            overrides: info
                .overrides
                .into_iter()
                .map(|o| AppliedOverride {
                    name: o.region,
                    index: o.index,
                    temperature: o.temperature.into(),
                    rain: o.rain.into(),
                })
                .collect(),
            budget: Budget {
                inflow: budget.inflow,
                source: budget.source,
                rain: budget.rain,
                outflow: budget.outflow,
            },
            mfd_stuck: info.mfd_stuck.unwrap_or(0),
        })
    }
}

/// `climate/meta.json`.
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct MetaJson {
    format: u32,
    #[allow(dead_code)] // read for the schema check, not used
    producer: String,
    dims: [usize; 2],
    sample_spacing_m: f64,
    origin_m: [f64; 2],
    files: std::collections::BTreeMap<String, String>,
    blake3: String,
}

/// `climate/climate.json` (`values` is the record of the declared values; the map does not keep it).
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct ClimateJson {
    format: u32,
    classes: Vec<String>,
    #[allow(dead_code)] // read for the schema check, not used
    values: serde_json::Value,
    overrides: Vec<OverrideJson>,
    budget: Option<BudgetJson>,
    mfd_stuck: Option<usize>,
}

#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct OverrideJson {
    region: String,
    index: u8,
    temperature: AdjustJson,
    rain: AdjustJson,
}

#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct AdjustJson {
    scale: f64,
    add: f64,
    pin: Option<f64>,
}

impl From<AdjustJson> for Adjust {
    fn from(a: AdjustJson) -> Self {
        Self { scale: a.scale, add: a.add, pin: a.pin }
    }
}

#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct BudgetJson {
    inflow: f64,
    source: f64,
    rain: f64,
    outflow: f64,
}

/// One `scrib-worldgen climate` invocation.
#[derive(Debug, Clone)]
pub struct ClimateRequest {
    /// Base guide folder (v1).
    pub base: PathBuf,
    /// Climate params TOML ([`ClimateFile::parse_toml`]); `None` = the declared defaults, no override.
    pub params: Option<PathBuf>,
    /// Class table TOML ([`ClassTable::parse_toml`]); `None` = the shipped Whittaker table.
    pub classes: Option<PathBuf>,
    /// `name=value` overrides of the declared values (after the params file).
    pub sets: Vec<(String, f64)>,
    /// Output folder (`climate/` is written inside it).
    pub out: PathBuf,
}

/// Value at fraction `f` of an ascending slice (nearest rank; NaN when empty).
fn quantile(sorted: &[f32], f: f64) -> f32 {
    if sorted.is_empty() {
        return f32::NAN;
    }
    sorted[((sorted.len() - 1) as f64 * f).round() as usize]
}

/// The values of `v` on land, ascending.
fn land_sorted(v: &[f32], land: &[bool]) -> Vec<f32> {
    let mut out: Vec<f32> = v.iter().zip(land).filter(|(_, l)| **l).map(|(x, _)| *x).collect();
    out.par_sort_unstable_by(f32::total_cmp);
    out
}

/// The summary line of one run: sizes, stage times, land statistics, the sweep budget, the hash.
fn summary(map: &ClimateMap, land: &[bool], times: &[StageTime], written_s: f64, hash: &str) -> String {
    let n_land = land.iter().filter(|l| **l).count();
    let t = land_sorted(&map.temperature, land);
    let p = land_sorted(&map.precipitation, land);
    let m = land_sorted(&map.moisture, land);
    let mut counts = vec![0usize; map.class_names.len()];
    for (&c, _) in map.classes.iter().zip(land).filter(|(_, l)| **l) {
        counts[usize::from(c)] += 1;
    }
    let shares: Vec<String> = map
        .class_names
        .iter()
        .zip(&counts)
        .filter(|(_, c)| **c > 0)
        .map(|(name, &c)| format!("{name} {:.1} %", 100.0 * c as f64 / n_land.max(1) as f64))
        .collect();
    let stages: Vec<String> = times.iter().map(|(name, s)| format!("{name} {s:.2}")).collect();
    let b = &map.budget;
    format!(
        "climate: {}x{} ({:.1} M samples, {:.1} % land); build {:.2} s ({}), write {written_s:.2} s; on land: \
         T {:.1}..{:.1} C, P p10 {:.0} p50 {:.0} p90 {:.0} p99 {:.0} max {:.0} mm/yr, moisture p10 {:.2} p50 {:.2} p90 {:.2}, classes {}; \
         {} overrides; budget (mm/yr km) inflow {:.4e} + source {:.4e} = rain {:.4e} + outflow {:.4e}; \
         mfd_stuck {}; blake3 {}",
        map.w,
        map.h,
        (map.w * map.h) as f64 * 1e-6,
        100.0 * n_land as f64 / (map.w * map.h).max(1) as f64,
        times.iter().map(|(_, s)| s).sum::<f64>(),
        stages.join(", "),
        quantile(&t, 0.0),
        quantile(&t, 1.0),
        quantile(&p, 0.1),
        quantile(&p, 0.5),
        quantile(&p, 0.9),
        quantile(&p, 0.99),
        quantile(&p, 1.0),
        quantile(&m, 0.1),
        quantile(&m, 0.5),
        quantile(&m, 0.9),
        shares.join(", "),
        map.overrides.len(),
        b.inflow,
        b.source,
        b.rain,
        b.outflow,
        map.mfd_stuck,
        &hash[..12]
    )
}

/// Reads the guide, the params file (or the defaults) and the class table (or the shipped one), applies the
/// `--set` values (ranges re-checked), builds the map, writes it and prints the summary line.
pub fn run_climate(req: &ClimateRequest) -> Result<ClimateMap> {
    let guide = Guide::read(&req.base)?;
    let mut file = match &req.params {
        Some(path) => ClimateFile::parse_toml(&std::fs::read_to_string(path)?)?,
        None => ClimateFile::defaults(),
    };
    for (k, v) in &req.sets {
        let d = file
            .declared
            .iter_mut()
            .find(|d| d.name == k)
            .ok_or_else(|| Error::Invalid(format!("climate: unknown value {k}")))?;
        d.v = *v;
        d.source = "Input(--set)".into();
    }
    file.climate = Climate::from_declared(&file.declared)?;
    let table = match &req.classes {
        Some(path) => ClassTable::parse_toml(&std::fs::read_to_string(path)?)?,
        None => ClassTable::whittaker(),
    };
    let (map, times) = build_timed(&guide, &file, &table)?;
    let clock = Instant::now();
    let hash = map.write(&req.out, &file.declared)?;
    let written_s = clock.elapsed().as_secs_f64();
    let land: Vec<bool> = guide.heights.iter().map(|&x| f64::from(x) > guide.sea_level_m).collect();
    println!("{}", summary(&map, &land, &times, written_s, &hash));
    Ok(map)
}
