//! **Landform census** (WG-L2a): natural landforms detected in a height field from the heights alone, by standard
//! geomorphometry (no learning), written as `features.json` + `overlay.png` + `README.md`.
//!
//! The generator must keep landmarks; this finds them in a guide (craters, basins, lakes, lagoons, trenches,
//! gorges, valleys, peaks, plateaus, bays, headlands, islands) so a names list can label them and the owner
//! confirm them, and later finds them again in a generated field to check they survived.
//!
//! Detectors (each module cites its technique):
//!
//! - [`depressions`]: closed depressions by Priority-Flood fill depth; `crater` / `basin` / `pool`.
//! - [`water`]: `lake`, `lagoon` (narrow-neck opening), `island`, `bay` and `headland` (morphological closing).
//! - [`channels`]: D8 channel network, main stems, valley cross-sections; `trench` / `gorge` / `valley`.
//! - [`relief`]: `peak` by topographic prominence, `plateau` by TPI + slope. Geomorphons and ridges are left out.
//!
//! **Analysis grid.** The guide is block-averaged by the integer factor nearest `analysis_spacing_m / spacing`
//! (≥ 1; a block is wet when most of its samples are, and takes that class's mean), so a 1.8 m dump and a 20 m
//! generated field are analysed at a few metres of the landmarks' scale.
//!
//! **Scale.** `scale` is the field's size over the reference world it was calibrated on (a 10x generated island
//! has `scale = 10`). Every declared value in metres is multiplied by `scale` and every one in m² by `scale²`, so
//! one set of thresholds serves both (the generated relief is scaled vertically too).
//!
//! **Names** (`--names`): a generic file `{format: 1, points: [{name, x_m, y_m, radius_m?, kind_hint?}]}` in the
//! guide frame. Each feature takes the nearest point within `max(radius_m, name_reach_m, name_reach_ratio ×
//! feature radius)` (distance to its polyline for channels, to its outline for areas, 0 inside it); a point names
//! several features only when they are of its `kind_hint` family (depression, water, channel, peak, plateau, bay,
//! headland, island), else at most one; a peak point names one peak, the most prominent in reach. Pairs are taken
//! best first (family match, then distance), so the join is deterministic.
//!
//! Deterministic: labelling, routing, stems and the prominence sweep are single-threaded in index order; `rayon`
//! only maps pure per-sample functions (hillshade, slope, distance transforms). Same input, same bytes.

pub mod channels;
pub mod depressions;
pub mod geom;
pub mod overlay;
pub mod relief;
pub mod water;

#[cfg(test)]
#[path = "../tests_landforms.rs"]
mod tests;

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::path::{Path, PathBuf};
use std::time::Instant;

use rayon::prelude::*;

use crate::filter::Declared;
use crate::guide::Guide;
use crate::{Error, Grid, Result};

fn decl(
    name: &'static str,
    v: f64,
    unit: &'static str,
    min: f64,
    max: f64,
    source: &str,
    help: &'static str,
) -> Declared {
    Declared {
        name,
        v,
        unit,
        min,
        max,
        source: source.into(),
        help,
    }
}

/// The declared census values with their defaults (reference-world values; `m` scale by `scale`, `m2` by
/// `scale²`).
#[allow(clippy::too_many_lines)]
pub fn declared_defaults() -> Vec<Declared> {
    vec![
        decl(
            "scale",
            1.0,
            "ratio",
            0.01,
            100.0,
            "Guess",
            "field size over the reference world (10 for a 10x generated island); multiplies every m and m2 value",
        ),
        decl(
            "analysis_spacing_m",
            4.0,
            "m",
            0.1,
            1000.0,
            "Guess",
            "target analysis spacing: the guide is block-averaged by the nearest integer factor (>= 1)",
        ),
        decl(
            "dep_min_depth_m",
            2.0,
            "m",
            0.0,
            1000.0,
            "Guess",
            "a closed depression counts when its fill depth reaches this",
        ),
        decl(
            "dep_min_area_m2",
            1500.0,
            "m2",
            0.0,
            1e9,
            "Guess",
            "and its filled area this (about a 44 m disc at scale 1)",
        ),
        decl(
            "crater_min_circularity",
            0.55,
            "ratio",
            0.0,
            1.0,
            "Guess",
            "crater: 4 pi A / P^2 at least this (1 = disc)",
        ),
        decl(
            "crater_min_depth_ratio",
            0.05,
            "ratio",
            0.0,
            1.0,
            "Reference(Pike 1977: fresh simple craters d/D ~0.2; 0.05 admits degraded and partly filled ones)",
            "crater: max depth / equivalent diameter at least this",
        ),
        decl(
            "crater_min_rim",
            0.7,
            "ratio",
            0.0,
            1.0,
            "Guess",
            "crater: share of rim rays with a crest (rim completeness)",
        ),
        decl(
            "rim_rays",
            24.0,
            "count",
            8.0,
            72.0,
            "Guess",
            "rays cast from the depression centroid for the rim",
        ),
        decl(
            "rim_width_ratio",
            1.0,
            "ratio",
            0.05,
            3.0,
            "Guess",
            "rim ring width beyond the depression edge (crest search; the flank is the lowest ground one width past the crest), share of r_eq",
        ),
        decl(
            "rim_rise_ratio",
            0.1,
            "ratio",
            0.0,
            2.0,
            "Guess",
            "a ray has a rim when its crest stands this share of the max depth above the outer flank",
        ),
        decl(
            "pool_max_depth_m",
            3.0,
            "m",
            0.0,
            100.0,
            "Guess",
            "pool: depression at most this deep",
        ),
        decl(
            "pool_max_floor_m",
            3.0,
            "m",
            0.0,
            100.0,
            "Guess",
            "pool: floor at most this above sea level",
        ),
        decl(
            "lake_min_area_m2",
            1500.0,
            "m2",
            0.0,
            1e9,
            "Guess",
            "lakes and lagoons at least this large",
        ),
        decl(
            "neck_width_m",
            40.0,
            "m",
            0.0,
            10000.0,
            "Guess",
            "sea water joined to the open sea only through necks narrower than this is a lagoon",
        ),
        decl(
            "channel_area_m2",
            40000.0,
            "m2",
            0.0,
            1e12,
            "Guess",
            "channels where the D8 drainage area reaches this",
        ),
        decl(
            "valley_halfwidth_m",
            80.0,
            "m",
            1.0,
            10000.0,
            "Guess",
            "valley cross-section reach each side of the channel",
        ),
        decl(
            "valley_min_depth_m",
            6.0,
            "m",
            0.0,
            1000.0,
            "Guess",
            "a channel stretch is a valley where its smoothed valley depth reaches this",
        ),
        decl(
            "valley_min_length_m",
            250.0,
            "m",
            0.0,
            1e6,
            "Guess",
            "valley runs at least this long",
        ),
        decl(
            "valley_smooth_m",
            40.0,
            "m",
            1.0,
            10000.0,
            "Guess",
            "along-stem window for the stem direction, the depth mean and bridged gaps",
        ),
        decl(
            "gorge_min_slope_deg",
            35.0,
            "deg",
            0.0,
            90.0,
            "Guess",
            "gorge: median wall slope (between 25 and 75 % of the depth) at least this",
        ),
        decl(
            "trench_min_floor_ratio",
            0.55,
            "ratio",
            0.0,
            1.0,
            "Guess",
            "trench: median width at 25 % over width at 75 % of the depth at least this (V = 1/3, box = 1)",
        ),
        decl(
            "peak_min_prominence_m",
            15.0,
            "m",
            0.0,
            10000.0,
            "Guess",
            "peaks at least this prominent (mountaineering lists use 30 m to 100 m at real scale)",
        ),
        decl(
            "plateau_radius_m",
            300.0,
            "m",
            1.0,
            1e5,
            "Guess",
            "TPI window half-width",
        ),
        decl(
            "plateau_min_tpi_m",
            10.0,
            "m",
            0.0,
            10000.0,
            "Guess",
            "plateau: TPI at least this",
        ),
        decl(
            "plateau_max_slope_deg",
            10.0,
            "deg",
            0.0,
            90.0,
            "Guess",
            "plateau: Horn slope at most this",
        ),
        decl(
            "plateau_min_area_m2",
            5000.0,
            "m2",
            0.0,
            1e12,
            "Guess",
            "plateau components at least this large",
        ),
        decl(
            "bay_radius_m",
            250.0,
            "m",
            1.0,
            1e5,
            "Guess",
            "closing radius of the coastline: bays and headlands are what it fills",
        ),
        decl(
            "bay_min_area_m2",
            40000.0,
            "m2",
            0.0,
            1e12,
            "Guess",
            "bays and headlands at least this large",
        ),
        decl(
            "bay_min_depth_ratio",
            0.3,
            "ratio",
            0.0,
            100.0,
            "Guess",
            "bays at least this deep (inland) for their mouth width",
        ),
        decl(
            "headland_min_depth_ratio",
            1.0,
            "ratio",
            0.0,
            100.0,
            "Guess",
            "headlands stick out at least this many times their base width (the closing also adds inlet corners)",
        ),
        decl(
            "island_min_area_m2",
            1500.0,
            "m2",
            0.0,
            1e12,
            "Guess",
            "islands at least this large",
        ),
        decl(
            "name_reach_m",
            120.0,
            "m",
            0.0,
            1e6,
            "Guess",
            "a name point reaches at least this far",
        ),
        decl(
            "name_reach_ratio",
            1.0,
            "ratio",
            0.0,
            100.0,
            "Guess",
            "and at least this many feature radii (sqrt(area / pi), half the length for channels)",
        ),
    ]
}

/// Census parameters, scaled ([`Params::from_declared`]).
#[derive(Debug, Clone, PartialEq)]
pub struct Params {
    pub scale: f64,
    pub analysis_spacing_m: f64,
    pub dep_min_depth_m: f64,
    pub dep_min_area_m2: f64,
    pub crater_min_circularity: f64,
    pub crater_min_depth_ratio: f64,
    pub crater_min_rim: f64,
    pub rim_rays: usize,
    pub rim_width_ratio: f64,
    pub rim_rise_ratio: f64,
    pub pool_max_depth_m: f64,
    pub pool_max_floor_m: f64,
    pub lake_min_area_m2: f64,
    pub neck_width_m: f64,
    pub channel_area_m2: f64,
    pub valley_halfwidth_m: f64,
    pub valley_min_depth_m: f64,
    pub valley_min_length_m: f64,
    pub valley_smooth_m: f64,
    pub gorge_min_slope_deg: f64,
    pub trench_min_floor_ratio: f64,
    pub peak_min_prominence_m: f64,
    pub plateau_radius_m: f64,
    pub plateau_min_tpi_m: f64,
    pub plateau_max_slope_deg: f64,
    pub plateau_min_area_m2: f64,
    pub bay_radius_m: f64,
    pub bay_min_area_m2: f64,
    pub bay_min_depth_ratio: f64,
    pub headland_min_depth_ratio: f64,
    pub island_min_area_m2: f64,
    pub name_reach_m: f64,
    pub name_reach_ratio: f64,
}

impl Params {
    /// Builds the parameters from declared values, scaling m by `scale` and m2 by `scale²`; checks ranges.
    pub fn from_declared(d: &[Declared]) -> Result<Self> {
        let scale = d.iter().find(|x| x.name == "scale").map_or(1.0, |x| x.v);
        let get = |name: &str| -> Result<f64> {
            let x = d
                .iter()
                .find(|x| x.name == name)
                .ok_or_else(|| Error::Invalid(format!("missing census value {name}")))?;
            if !(x.min..=x.max).contains(&x.v) {
                return Err(Error::Invalid(format!(
                    "{name} = {} outside [{}, {}]",
                    x.v, x.min, x.max
                )));
            }
            Ok(match x.unit {
                "m" => x.v * scale,
                "m2" => x.v * scale * scale,
                _ => x.v,
            })
        };
        Ok(Self {
            scale,
            analysis_spacing_m: get("analysis_spacing_m")?,
            dep_min_depth_m: get("dep_min_depth_m")?,
            dep_min_area_m2: get("dep_min_area_m2")?,
            crater_min_circularity: get("crater_min_circularity")?,
            crater_min_depth_ratio: get("crater_min_depth_ratio")?,
            crater_min_rim: get("crater_min_rim")?,
            rim_rays: get("rim_rays")?.round() as usize,
            rim_width_ratio: get("rim_width_ratio")?,
            rim_rise_ratio: get("rim_rise_ratio")?,
            pool_max_depth_m: get("pool_max_depth_m")?,
            pool_max_floor_m: get("pool_max_floor_m")?,
            lake_min_area_m2: get("lake_min_area_m2")?,
            neck_width_m: get("neck_width_m")?,
            channel_area_m2: get("channel_area_m2")?,
            valley_halfwidth_m: get("valley_halfwidth_m")?,
            valley_min_depth_m: get("valley_min_depth_m")?,
            valley_min_length_m: get("valley_min_length_m")?,
            valley_smooth_m: get("valley_smooth_m")?,
            gorge_min_slope_deg: get("gorge_min_slope_deg")?,
            trench_min_floor_ratio: get("trench_min_floor_ratio")?,
            peak_min_prominence_m: get("peak_min_prominence_m")?,
            plateau_radius_m: get("plateau_radius_m")?,
            plateau_min_tpi_m: get("plateau_min_tpi_m")?,
            plateau_max_slope_deg: get("plateau_max_slope_deg")?,
            plateau_min_area_m2: get("plateau_min_area_m2")?,
            bay_radius_m: get("bay_radius_m")?,
            bay_min_area_m2: get("bay_min_area_m2")?,
            bay_min_depth_ratio: get("bay_min_depth_ratio")?,
            headland_min_depth_ratio: get("headland_min_depth_ratio")?,
            island_min_area_m2: get("island_min_area_m2")?,
            name_reach_m: get("name_reach_m")?,
            name_reach_ratio: get("name_reach_ratio")?,
        })
    }
}

/// Sets one declared value (source `Guess` unless given).
pub fn set(d: &mut [Declared], name: &str, v: f64, source: Option<String>) -> Result<()> {
    let p = d
        .iter_mut()
        .find(|p| p.name == name)
        .ok_or_else(|| Error::Invalid(format!("unknown census value {name}")))?;
    p.v = v;
    p.source = source.unwrap_or_else(|| "Guess".into());
    Ok(())
}

/// Guide-frame placement of the analysis grid: sample (col, row) at `origin + (col, −row) × spacing`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Frame {
    pub origin_m: [f64; 2],
    pub spacing: f64,
}

impl Frame {
    /// Guide-frame metres of a fractional sample position.
    pub fn to_m(&self, col: f64, row: f64) -> [f64; 2] {
        [
            self.origin_m[0] + col * self.spacing,
            self.origin_m[1] - row * self.spacing,
        ]
    }

    /// Fractional sample position of a guide-frame point.
    pub fn to_sample(&self, p: [f64; 2]) -> (f64, f64) {
        (
            (p[0] - self.origin_m[0]) / self.spacing,
            (self.origin_m[1] - p[1]) / self.spacing,
        )
    }
}

/// The analysed field.
#[derive(Debug, Clone)]
pub struct Field {
    pub g: Grid,
    pub wet: Vec<bool>,
    pub sea: f64,
    pub frame: Frame,
}

impl Field {
    /// A field from a grid whose sample (0, 0) sits at `origin_m`.
    pub fn new(g: Grid, sea: f64, origin_m: [f64; 2]) -> Self {
        let wet = g.data.iter().map(|&v| v < sea).collect();
        let frame = Frame {
            origin_m,
            spacing: g.spacing,
        };
        Self { g, wet, sea, frame }
    }

    /// Block-mean coarsening by `k` (wet when most samples are; that class's mean); origin at the block centre.
    #[must_use]
    pub fn coarsen(&self, k: usize) -> Self {
        if k <= 1 {
            return self.clone();
        }
        let g = &self.g;
        let (w, h) = (g.w / k, g.h / k);
        let mut data = vec![0.0; w * h];
        data.par_chunks_mut(w).enumerate().for_each(|(by, row)| {
            for (bx, v) in row.iter_mut().enumerate() {
                let (mut sl, mut nl, mut sw, mut nw) = (0.0, 0usize, 0.0, 0usize);
                for y in by * k..(by + 1) * k {
                    for x in bx * k..(bx + 1) * k {
                        let z = g.data[y * g.w + x];
                        if z < self.sea {
                            sw += z;
                            nw += 1;
                        } else {
                            sl += z;
                            nl += 1;
                        }
                    }
                }
                *v = if nw > nl {
                    sw / nw as f64
                } else {
                    sl / nl as f64
                };
            }
        });
        let shift = (k - 1) as f64 * 0.5 * g.spacing;
        let origin = [
            self.frame.origin_m[0] + shift,
            self.frame.origin_m[1] - shift,
        ];
        Self::new(
            Grid {
                w,
                h,
                spacing: g.spacing * k as f64,
                data,
            },
            self.sea,
            origin,
        )
    }
}

/// One detected landform, guide-frame metres.
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct Feature {
    pub id: String,
    pub kind: &'static str,
    pub confidence: f64,
    pub name: Option<String>,
    pub centroid_m: [f64; 2],
    /// `[x_min, y_min, x_max, y_max]`.
    pub bbox_m: [f64; 4],
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outline_m: Option<Vec<[f64; 2]>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub polyline_m: Option<Vec<[f64; 2]>>,
    pub metrics: BTreeMap<&'static str, f64>,
    /// Sort key within a kind (area, length × depth, prominence), largest first.
    #[serde(skip)]
    pub size: f64,
}

/// Kinds in output order.
// tunables: const (ids: the kind names in output order)
pub const KINDS: [&str; 13] = [
    "crater", "basin", "pool", "lake", "lagoon", "trench", "gorge", "valley", "peak", "plateau",
    "bay", "headland", "island",
];

/// Name-join family of a kind (a `kind_hint` names every feature of its family).
pub fn family(kind: &str) -> &'static str {
    match kind {
        "crater" | "basin" | "pool" => "depression",
        "lake" | "lagoon" => "water",
        "trench" | "gorge" | "valley" => "channel",
        "peak" => "peak",
        "plateau" => "plateau",
        "bay" => "bay",
        "headland" => "headland",
        "island" => "island",
        _ => "other",
    }
}

fn r3(v: f64) -> f64 {
    (v * 1000.0).round() / 1000.0
}

fn r2(v: f64) -> f64 {
    (v * 100.0).round() / 100.0
}

/// Confidence of a metric `v` against its threshold `t`: 0.5 at the threshold, toward 1 well above it.
pub fn score(v: f64, t: f64) -> f64 {
    if t <= 0.0 {
        return 1.0;
    }
    if v <= 0.0 {
        return 0.0;
    }
    (0.5 + 0.5 * (2.0 * (v / t).ln()).tanh()).clamp(0.0, 1.0)
}

/// Builds a feature from sample-space geometry (outlines simplified to ≤ 200 vertices).
#[allow(clippy::too_many_arguments)]
pub fn feature(
    kind: &'static str,
    confidence: f64,
    f: &Field,
    centroid: [f64; 2],
    bbox: [f64; 4],
    outline: Option<Vec<[f64; 2]>>,
    polyline: Option<Vec<[f64; 2]>>,
    metrics: Vec<(&'static str, f64)>,
    size: f64,
) -> Feature {
    let fr = f.frame;
    let conv = |v: Vec<[f64; 2]>| -> Vec<[f64; 2]> {
        v.into_iter()
            .map(|p| {
                let m = fr.to_m(p[0], p[1]);
                [r2(m[0]), r2(m[1])]
            })
            .collect()
    };
    let outline = outline.map(|o| conv(geom::simplify(&o, true, 0.75, 200)));
    let polyline = polyline.map(conv);
    let c = fr.to_m(centroid[0], centroid[1]);
    let (a, b) = (fr.to_m(bbox[0], bbox[3]), fr.to_m(bbox[2], bbox[1]));
    Feature {
        id: String::new(),
        kind,
        confidence: r3(confidence),
        name: None,
        centroid_m: [r2(c[0]), r2(c[1])],
        bbox_m: [r2(a[0]), r2(a[1]), r2(b[0]), r2(b[1])],
        outline_m: outline,
        polyline_m: polyline,
        metrics: metrics.into_iter().map(|(k, v)| (k, r3(v))).collect(),
        size,
    }
}

/// Sorts by kind (in [`KINDS`] order), then size (largest first), then position; assigns ids `kind-N`.
pub fn sort_and_number(features: &mut [Feature]) {
    let rank = |k: &str| KINDS.iter().position(|&x| x == k).unwrap_or(KINDS.len());
    features.sort_by(|a, b| {
        rank(a.kind)
            .cmp(&rank(b.kind))
            .then(b.size.total_cmp(&a.size))
            .then(a.centroid_m[0].total_cmp(&b.centroid_m[0]))
            .then(a.centroid_m[1].total_cmp(&b.centroid_m[1]))
    });
    let mut counts: BTreeMap<&str, usize> = BTreeMap::new();
    for f in features.iter_mut() {
        let c = counts.entry(f.kind).or_insert(0);
        *c += 1;
        f.id = format!("{}-{}", f.kind, c);
    }
}

/// One name point.
#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct NamePoint {
    pub name: String,
    pub x_m: f64,
    pub y_m: f64,
    #[serde(default)]
    pub radius_m: Option<f64>,
    #[serde(default)]
    pub kind_hint: Option<String>,
}

#[derive(serde::Deserialize)]
struct NamesFile {
    format: u32,
    points: Vec<NamePoint>,
}

/// Reads a names file (format 1).
pub fn read_names(path: &Path) -> Result<Vec<NamePoint>> {
    let f: NamesFile = serde_json::from_str(&std::fs::read_to_string(path)?)?;
    if f.format != 1 {
        return Err(Error::Invalid(format!(
            "names format {} (1 expected)",
            f.format
        )));
    }
    Ok(f.points)
}

/// Feature radius for the name reach: `sqrt(area / pi)`, half the length for channels, 0 for points.
fn feature_radius(f: &Feature) -> f64 {
    if let Some(a) = f.metrics.get("area_m2") {
        return (a / std::f64::consts::PI).sqrt();
    }
    f.metrics.get("length_m").map_or(0.0, |l| l / 2.0)
}

/// Joins names to features (see the module docs).
pub fn join_names(features: &mut [Feature], points: &[NamePoint], p: &Params) {
    // (family mismatch, distance (or −prominence for a peak point on a peak), feature, point)
    let mut pairs: Vec<(bool, f64, usize, usize)> = Vec::new();
    for (fi, f) in features.iter().enumerate() {
        let reach_f = p.name_reach_m.max(p.name_reach_ratio * feature_radius(f));
        for (pi, pt) in points.iter().enumerate() {
            let q = [pt.x_m, pt.y_m];
            let d = match (&f.polyline_m, &f.outline_m) {
                (Some(line), _) => geom::polyline_dist(q, line),
                (None, Some(o)) if geom::inside_polygon(q, o) => 0.0,
                (None, Some(o)) => {
                    let mut ring = o.clone();
                    ring.push(o[0]);
                    geom::polyline_dist(q, &ring)
                }
                (None, None) => (q[0] - f.centroid_m[0]).hypot(q[1] - f.centroid_m[1]),
            };
            let reach = reach_f.max(pt.radius_m.unwrap_or(0.0));
            if d <= reach {
                let matches = pt
                    .kind_hint
                    .as_deref()
                    .is_some_and(|h| family(h) == family(f.kind));
                // A summit has one name: a peak point goes to the most prominent peak in reach.
                let key = if matches && f.kind == "peak" {
                    -f.size
                } else {
                    d
                };
                pairs.push((!matches, key, fi, pi));
            }
        }
    }
    pairs.sort_by(|a, b| {
        a.0.cmp(&b.0)
            .then(a.1.total_cmp(&b.1))
            .then(a.2.cmp(&b.2))
            .then(a.3.cmp(&b.3))
    });
    let mut used = vec![false; points.len()];
    for (mismatch, _, fi, pi) in pairs {
        if features[fi].name.is_some() {
            continue;
        }
        let hinted = !mismatch;
        if (!hinted || features[fi].kind == "peak") && used[pi] {
            continue;
        }
        // A hinted point of another family never names this feature, and a plain point (a site marker) does not
        // repeat a name a hinted point already gave.
        if !hinted
            && (points[pi].kind_hint.is_some()
                || features
                    .iter()
                    .any(|f| f.name.as_deref() == Some(points[pi].name.as_str())))
        {
            continue;
        }
        features[fi].name = Some(points[pi].name.clone());
        used[pi] = true;
    }
}

/// A census run.
#[derive(Debug, Clone)]
pub struct LandformsRequest {
    pub guide: PathBuf,
    pub names: Option<PathBuf>,
    pub out: PathBuf,
    /// Overrides the guide's `origin_m`.
    pub origin_m: Option<[f64; 2]>,
    pub declared: Vec<Declared>,
    /// The command line, for the README.
    pub command: String,
}

/// What a run produced.
#[derive(Debug, Clone)]
pub struct Census {
    pub features: Vec<Feature>,
    pub timings: Vec<(&'static str, f64)>,
    pub field: Field,
    pub json: String,
}

/// Runs every detector on `field` and returns the sorted, numbered features and per-detector timings.
pub fn census(field: &Field, p: &Params) -> (Vec<Feature>, Vec<(&'static str, f64)>) {
    let mut timings = Vec::new();
    let mut all = Vec::new();
    let t = Instant::now();
    let (dep, fill_depth) = depressions::detect(field, p);
    timings.push(("depressions", t.elapsed().as_secs_f64()));
    all.extend(dep);
    let t = Instant::now();
    let (wat, route_wet) = water::detect(field, p);
    all.extend(wat);
    timings.push(("water and coast", t.elapsed().as_secs_f64()));
    let t = Instant::now();
    all.extend(channels::detect(field, p, &fill_depth, &route_wet));
    timings.push(("channels", t.elapsed().as_secs_f64()));
    let t = Instant::now();
    all.extend(relief::peaks(field, p));
    timings.push(("peaks", t.elapsed().as_secs_f64()));
    let t = Instant::now();
    all.extend(relief::plateaus(field, p));
    timings.push(("plateaus", t.elapsed().as_secs_f64()));
    sort_and_number(&mut all);
    (all, timings)
}

/// `features.json` text of a census.
pub fn features_json(
    guide: &str,
    field: &Field,
    src_spacing: f64,
    declared: &[Declared],
    features: &[Feature],
) -> Result<String> {
    let v = serde_json::json!({
        "format": 1,
        "guide": guide,
        "spacing_m": field.g.spacing,
        "source_spacing_m": src_spacing,
        "frame": {
            "origin_m": field.frame.origin_m,
            "orientation": "guide frame: x east, y north, metres; analysis sample (col, row) at origin_m + (col, -row) x spacing_m",
            "dims": [field.g.w, field.g.h],
            "sea_level_m": field.sea,
        },
        "declared": declared,
        "features": features,
    });
    let mut s = serde_json::to_string_pretty(&v)?;
    s.push('\n');
    Ok(s)
}

/// Reads the guide, runs the census, joins names, writes `features.json`, `overlay.png` and `README.md`.
pub fn run_landforms(req: &LandformsRequest) -> Result<Census> {
    let t0 = Instant::now();
    let p = Params::from_declared(&req.declared)?;
    let guide = Guide::read(&req.guide)?;
    let origin = req.origin_m.unwrap_or(guide.origin_m);
    let src = Field::new(
        Grid {
            w: guide.w,
            h: guide.h,
            spacing: guide.spacing_m,
            data: guide.heights.iter().map(|&v| f64::from(v)).collect(),
        },
        guide.sea_level_m,
        origin,
    );
    let k = (p.analysis_spacing_m / guide.spacing_m).round().max(1.0) as usize;
    let field = src.coarsen(k);
    drop(src);
    let t_load = t0.elapsed().as_secs_f64();
    let (mut features, mut timings) = census(&field, &p);
    timings.insert(0, ("load + coarsen", t_load));
    if let Some(np) = &req.names {
        let pts = read_names(np)?;
        let t = Instant::now();
        join_names(&mut features, &pts, &p);
        timings.push(("names", t.elapsed().as_secs_f64()));
    }
    std::fs::create_dir_all(&req.out)?;
    let guide_name = req.guide.to_string_lossy().replace('\\', "/");
    let json = features_json(
        &guide_name,
        &field,
        guide.spacing_m,
        &req.declared,
        &features,
    )?;
    std::fs::write(req.out.join("features.json"), &json)?;
    let t = Instant::now();
    overlay::write(&req.out.join("overlay.png"), &field, &features, 2400)?;
    timings.push(("overlay", t.elapsed().as_secs_f64()));
    let readme = readme(req, &field, guide.spacing_m, k, &features, &timings);
    std::fs::write(req.out.join("README.md"), readme)?;
    Ok(Census {
        features,
        timings,
        field,
        json,
    })
}

fn key_metrics(f: &Feature) -> String {
    let keys: &[&str] = match family(f.kind) {
        "depression" => &[
            "eq_diameter_m",
            "max_depth_m",
            "floor_m",
            "spill_m",
            "circularity",
            "rim_completeness",
        ],
        "water" => &["area_m2", "max_depth_m"],
        "channel" => &[
            "length_m",
            "mean_depth_m",
            "width_half_depth_m",
            "wall_slope_deg",
            "floor_ratio",
        ],
        "peak" => &["height_m", "prominence_m"],
        "plateau" => &["area_m2", "mean_height_m", "mean_tpi_m"],
        "coast" => &["area_m2", "mouth_width_m", "inland_depth_m"],
        _ => &["area_m2", "max_height_m"],
    };
    keys.iter()
        .filter_map(|k| f.metrics.get(k).map(|v| format!("{k} {v}")))
        .collect::<Vec<_>>()
        .join(", ")
}

fn readme(
    req: &LandformsRequest,
    field: &Field,
    src_spacing: f64,
    k: usize,
    features: &[Feature],
    timings: &[(&'static str, f64)],
) -> String {
    let mut s = String::new();
    let _ = writeln!(s, "# Landform census\n");
    let _ = writeln!(s, "Command: `{}`\n", req.command);
    let _ = writeln!(
        s,
        "Guide `{}`: {} m spacing, analysed at {} m ({k}x{k} block mean), {} x {} samples, origin ({}, {}) m.\n",
        req.guide.display(),
        src_spacing,
        field.g.spacing,
        field.g.w,
        field.g.h,
        field.frame.origin_m[0],
        field.frame.origin_m[1]
    );
    let _ = writeln!(s, "## Timings\n\n| Detector | s |\n|---|---|");
    for (n, t) in timings {
        let _ = writeln!(s, "| {n} | {t:.2} |");
    }
    let _ = writeln!(s, "\n## Counts\n\n| Kind | Count | Named |\n|---|---|---|");
    for kind in KINDS {
        let c = features.iter().filter(|f| f.kind == kind).count();
        let nm = features
            .iter()
            .filter(|f| f.kind == kind && f.name.is_some())
            .count();
        let _ = writeln!(s, "| {kind} | {c} | {nm} |");
    }
    let _ = writeln!(
        s,
        "\n## Overlay legend\n\n`overlay.png`: hillshade, each feature in its kind's colour with its number (the N of `kind-N`): {}.\n",
        KINDS
            .iter()
            .map(|k| {
                let c = overlay::colour(k);
                format!("{k} #{:02x}{:02x}{:02x}", c[0], c[1], c[2])
            })
            .collect::<Vec<_>>()
            .join(", ")
    );
    let _ = writeln!(s, "## Top features per kind\n");
    for kind in KINDS {
        let top: Vec<&Feature> = features.iter().filter(|f| f.kind == kind).take(8).collect();
        if top.is_empty() {
            continue;
        }
        let _ = writeln!(
            s,
            "### {kind}\n\n| id | name | centre m | conf | metrics |\n|---|---|---|---|---|"
        );
        for f in top {
            let _ = writeln!(
                s,
                "| {} | {} | ({:.0}, {:.0}) | {} | {} |",
                f.id,
                f.name.as_deref().unwrap_or("-"),
                f.centroid_m[0],
                f.centroid_m[1],
                f.confidence,
                key_metrics(f)
            );
        }
        let _ = writeln!(s);
    }
    s
}
