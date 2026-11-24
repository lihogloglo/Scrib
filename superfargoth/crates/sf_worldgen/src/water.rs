//! **Lakes and rivers of a guide, format v1**: reads the water network an offline tool writes into a guide folder
//! (`worldgen-water`). Bevy-free and generic: a lake is a polygon at a level, a river a polyline with a width and a
//! surface level per point; the game builds their meshes (`sf_water::body_mesh`).
//!
//! **File.** `<guide>/water.json`: `{ "format": 1, "sea_level_m", "lakes": [{ "id", "level_m", "outline": [[x, y],
//! ..], "holes": [[[x, y], ..]], .. }], "rivers": [{ "id", "kind", "points": [[x, y, width_m, level_m], ..], .. }],
//! .. }`, guide frame (x east, y north, metres; [`crate::guide`]). River points run upstream to downstream. Extra
//! keys (`frame`, `stats`, `area_m2`, `joins`, ...) are ignored. [`to_engine_xz`] maps a guide point to the
//! engine's XZ ([`crate::tiles`] "Frames").

use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use serde::Deserialize;

use crate::{Error, Result};

/// The `water.json` format this module reads.
pub const FORMAT: u32 = 1;

/// A lake: a closed outline (either winding, closing point optional) with holes, at one surface level.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Lake {
    #[serde(default)]
    pub id: i64,
    /// Surface level, metres.
    pub level_m: f64,
    /// Guide frame, metres.
    pub outline: Vec<[f64; 2]>,
    #[serde(default)]
    pub holes: Vec<Vec<[f64; 2]>>,
}

/// A river (or a lake link, a braid, a distributary: `kind` as written): its centre line upstream to downstream.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct River {
    #[serde(default)]
    pub id: i64,
    #[serde(default)]
    pub kind: String,
    /// `[x, y, width_m, level_m]`, guide frame, metres.
    pub points: Vec<[f64; 4]>,
}

/// The water network of a guide (format v1).
#[derive(Debug, Clone, PartialEq, Default)]
pub struct GuideWater {
    pub sea_level_m: f64,
    pub lakes: Vec<Lake>,
    pub rivers: Vec<River>,
}

#[derive(Deserialize)]
struct FileV1 {
    format: u32,
    #[serde(default)]
    sea_level_m: f64,
    #[serde(default)]
    lakes: Vec<Lake>,
    #[serde(default)]
    rivers: Vec<River>,
}

impl GuideWater {
    /// Reads `water.json` from a guide folder; `None` when it is absent.
    pub fn read(dir: &Path) -> Result<Option<Self>> {
        let path = dir.join("water.json");
        if !path.exists() {
            return Ok(None);
        }
        Self::from_reader(BufReader::new(File::open(path)?)).map(Some)
    }

    /// Parses a `water.json` document.
    pub fn from_reader(r: impl std::io::Read) -> Result<Self> {
        let file: FileV1 = serde_json::from_reader(r)?;
        if file.format != FORMAT {
            return Err(Error::Invalid(format!("water format {} (v{FORMAT} expected)", file.format)));
        }
        Ok(Self { sea_level_m: file.sea_level_m, lakes: file.lakes, rivers: file.rivers })
    }

    /// Total river centre-line length, metres (the polyline's, before any smoothing).
    #[must_use]
    pub fn river_length_m(&self) -> f64 {
        self.rivers
            .iter()
            .flat_map(|r| r.points.windows(2))
            .map(|w| (w[1][0] - w[0][0]).hypot(w[1][1] - w[0][1]))
            .sum()
    }
}

/// A guide point `(x east, y north)` as engine `(X, Z)`: `(x, −y)` ([`crate::tiles`] "Frames").
#[must_use]
pub fn to_engine_xz(p: [f64; 2]) -> [f64; 2] {
    [p[0], -p[1]]
}
