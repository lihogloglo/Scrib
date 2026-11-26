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
//! `meta.json` (dims, spacing, origin, per-file and product blake3).

use std::path::Path;

use rayon::prelude::*;

use crate::climate::{moisture, precipitation, temperature, Budget, ClimateFile};
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

/// Builds the climate product of `guide` (module docs for the order).
pub fn build(guide: &Guide, file: &ClimateFile, table: &ClassTable) -> Result<ClimateMap> {
    let c = &file.climate;
    let (w, h) = (guide.w, guide.h);
    let heights = Grid {
        w,
        h,
        spacing: guide.spacing_m,
        data: guide.heights.iter().map(|&x| f64::from(x) - guide.sea_level_m).collect(),
    };
    let t = temperature(&heights, c);
    let rain = precipitation(&heights, &t, c)?;

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

    let wet: Vec<bool> = heights.data.iter().map(|&x| x <= 0.0).collect();
    let mut z = Vec::new();
    priority_flood_eps(w, h, &heights.data, &wet, 0.0, &mut z);
    let (area, mfd_stuck) = mfd_area(w, h, heights.spacing, &z, &wet, c.drain_flow_exponent);

    let m = moisture(&t, &p, &area, c)?;
    let cls = classes(&t, &p, table)?;
    let to_f32 = |g: &Grid| g.data.iter().map(|&x| x as f32).collect::<Vec<f32>>();
    Ok(ClimateMap {
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
    })
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
}
