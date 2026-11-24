//! **Road networks, format v1, and draped road ribbons** (slice GR-1a): reads the regional road network an offline
//! router writes into a guide folder and builds ribbon mesh data that conforms to a height function. Bevy-free:
//! the game turns [`RibbonChunk`]s into meshes. Generic: a road is a classed polyline with a width; nothing here
//! names a game.
//!
//! **File.** `<guide>/roads.json`: `{ "format": 1, "segments": [{ "id", "class", "surface", "width_m", "a", "b",
//! "points": [[x, y], ..], "crossings": [{ "kind", "points": [[x, y], ..] }], "length_m", "grade" }],
//! "junctions": [{ "id", "pos": [x, y], "kind", "sites": [key] }], "warnings": [..], "unreachable": [..] }`, guide
//! frame (x east, y north, metres; [`crate::guide`]). Classes `highway | main | local | track | trail`; crossing
//! kinds `bridge | ford | causeway | access`; an unknown class or kind reads as `Other`. Extra keys are ignored.
//!
//! **Ribbon** (the industry road-spline recipe: UE Landscape Splines' spline meshes, Horizon-style road ribbons):
//!
//! 1. **Centreline.** The coarse router polyline (points every ~60-120 m) is smoothed by an *interpolating*
//!    centripetal Catmull-Rom spline (alpha 0.5, Yuksel, Schaefer, Keyser 2011: no cusps or self-intersections
//!    within a piece), written per piece as a cubic Hermite with the non-uniform Catmull-Rom tangents and phantom
//!    end points reflected through the ends. The curve passes through every input point.
//! 2. **Arc length.** A dense table of the spline ([`ARC_SUBSTEPS_PER_SPACING`] samples per `spacing_m` of chord)
//!    maps arc length `s` to a piece parameter; positions and tangents are evaluated on the spline itself.
//! 3. **Spans.** Each `bridge` / `causeway` crossing is projected onto the centreline (its first and last point,
//!    nearest arc length), overlapping intervals merge, and the ribbon skips them ([`Span`], decks are a later
//!    slice). Fords and access tracks stay on the ground.
//! 4. **Rows.** Each ground run between spans is resampled at a uniform step of at most `spacing_m` (both run ends
//!    included). A row is `cross_samples` vertices across the width: column 0 on the left edge, the last on the
//!    right edge (left = 90 degrees counter-clockwise of the direction of travel, seen from above), columns 1 and
//!    `n − 2` on the shoulder lines (`min(shoulder_m, shoulder_max_frac · width)` inside the edges), the rest even
//!    in between. Every vertex sits `lift_m` above `height(x, y)` at its own XY, so the ribbon conforms across
//!    the width as well as along it.
//! 5. **Attributes.** Normals by central differences on the row/column grid of the whole run (so they agree at chunk
//!    seams), oriented up; `u` across 0 (left) to 1 (right); `v = s / texture_length_m` with `s` from the segment's
//!    start (continuous across chunks and spans); edge `fade` 0 on the edge columns, 1 from the shoulder lines
//!    inward (a linear ramp over the shoulder once interpolated).
//! 6. **Chunks.** A run is cut into chunks of at most `chunk_m` along, adjacent chunks sharing their seam row
//!    (identical positions, normals, uvs and fade), so the game can cull them with no gap. Positions are f32
//!    relative to the chunk's [`RibbonChunk::origin`] (f64, guide frame, z up). Triangles wind counter-clockwise
//!    seen from above (+z); the map to a y-up frame `(x, y, z) → (x, z, −y)` is a rotation and keeps that winding.

use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use serde::Deserialize;

use crate::filter::Declared;
use crate::{Error, Result};

#[cfg(test)]
#[path = "tests_roads.rs"]
mod tests;

/// The `roads.json` format this module reads.
pub const FORMAT: u32 = 1;

/// Dense arc-length table samples per `spacing_m` of chord.
// tunables: const (maths: quadrature density of the arc-length table, accuracy only; trades no look or cost)
pub const ARC_SUBSTEPS_PER_SPACING: f64 = 8.0;

/// Points closer than this are one point (the spline needs distinct knots).
// tunables: const (maths: duplicate-point epsilon, metres)
const DUP_EPS_M: f64 = 1e-6;

/// Road class (the router's hierarchy).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RoadClass {
    Highway,
    Main,
    Local,
    Track,
    Trail,
    #[serde(other)]
    Other,
}

/// Kind of a crossing span along a segment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CrossingKind {
    Bridge,
    Ford,
    Causeway,
    /// A farm track's entry into a place's fields.
    Access,
    #[serde(other)]
    Other,
}

impl CrossingKind {
    /// True for the kinds the ribbon skips (a deck carries the road there).
    pub fn is_deck(self) -> bool {
        matches!(self, Self::Bridge | Self::Causeway)
    }
}

/// One crossing span of a segment.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Crossing {
    pub kind: CrossingKind,
    /// Guide-frame polyline of the span, metres.
    pub points: Vec<[f64; 2]>,
}

/// One road segment between two junctions.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Segment {
    pub id: u32,
    pub class: RoadClass,
    #[serde(default)]
    pub surface: String,
    /// Full width of the travelled way, metres.
    pub width_m: f64,
    /// Junction ids at both ends.
    pub a: u32,
    pub b: u32,
    /// Guide-frame centreline polyline, metres (coarse: routing-grid spacing).
    pub points: Vec<[f64; 2]>,
    #[serde(default)]
    pub crossings: Vec<Crossing>,
    /// Length the router measured, metres.
    #[serde(default)]
    pub length_m: f64,
    /// Steepest grade along the path (rise over run).
    #[serde(default)]
    pub grade: f64,
}

/// One junction (a segment end).
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Junction {
    pub id: u32,
    pub pos: [f64; 2],
    /// `site`, `access`, `exit`, `junction` (the router's labels).
    #[serde(default)]
    pub kind: String,
    /// Site keys at this junction.
    #[serde(default)]
    pub sites: Vec<String>,
}

/// The road network of a guide (format v1).
#[derive(Debug, Clone, PartialEq, Default)]
pub struct RoadNetwork {
    pub segments: Vec<Segment>,
    pub junctions: Vec<Junction>,
    pub warnings: Vec<String>,
    /// Site keys no road reaches.
    pub unreachable: Vec<String>,
}

#[derive(Deserialize)]
struct FileV1 {
    format: u32,
    #[serde(default)]
    segments: Vec<Segment>,
    #[serde(default)]
    junctions: Vec<Junction>,
    #[serde(default)]
    warnings: Vec<String>,
    #[serde(default)]
    unreachable: Vec<String>,
}

impl RoadNetwork {
    /// Reads `roads.json` from a guide folder; `None` when it is absent.
    pub fn read(dir: &Path) -> Result<Option<Self>> {
        let path = dir.join("roads.json");
        if !path.exists() {
            return Ok(None);
        }
        Self::from_reader(BufReader::new(File::open(path)?)).map(Some)
    }

    /// Parses a `roads.json` document.
    pub fn from_reader(r: impl std::io::Read) -> Result<Self> {
        let file: FileV1 = serde_json::from_reader(r)?;
        if file.format != FORMAT {
            return Err(Error::Invalid(format!("roads format {} (v{FORMAT} expected)", file.format)));
        }
        let net = Self {
            segments: file.segments,
            junctions: file.junctions,
            warnings: file.warnings,
            unreachable: file.unreachable,
        };
        for s in &net.segments {
            s.validate()?;
        }
        Ok(net)
    }
}

impl Segment {
    fn validate(&self) -> Result<()> {
        let bad = |m: &str| Err(Error::Invalid(format!("road segment {}: {m}", self.id)));
        if self.points.len() < 2 {
            return bad("fewer than 2 points");
        }
        let finite = |p: &[f64; 2]| p[0].is_finite() && p[1].is_finite();
        if !self.points.iter().all(finite) || !self.crossings.iter().flat_map(|c| &c.points).all(finite) {
            return bad("non-finite point");
        }
        if !(self.width_m.is_finite() && self.width_m > 0.0) {
            return bad("width_m must be > 0");
        }
        Ok(())
    }
}

/// Declared ribbon values with their defaults (spec `tunables.md` "Definition").
pub fn declared_defaults() -> Vec<Declared> {
    let d = |name, v, unit, min, max, source: &str, help| Declared {
        name,
        v,
        unit,
        min,
        max,
        source: source.into(),
        help,
    };
    vec![
        d("spacing_m", 2.0, "m", 0.25, 50.0, "Guess(a few 0.5 m terrain samples; the detail filter's shortest octave is 20 m)",
            "largest step between ribbon rows along the centreline"),
        d("chunk_m", 256.0, "m", 8.0, 4096.0, "Guess(on the order of a terrain tile, for culling)",
            "longest ribbon chunk along the centreline"),
        d("cross_samples", 5.0, "count", 3.0, 33.0, "Guess(edges + shoulder lines + centre)",
            "vertices across the width of a row"),
        d("lift_m", 0.08, "m", 0.0, 2.0, "Guess(decal-style offset; the material adds a depth bias)",
            "height of the ribbon above the height function"),
        d("shoulder_m", 0.6, "m", 0.0, 5.0, "Guess",
            "width of the edge fade (0 at the edge, 1 inside)"),
        d("shoulder_max_frac", 0.3, "ratio", 0.0, 0.5, "Guess(keeps a trail's solid core)",
            "cap on the shoulder as a fraction of the road width"),
        d("texture_length_m", 4.0, "m", 0.1, 100.0, "Guess",
            "metres along the road per texture repeat (v)"),
    ]
}

/// Checked ribbon values (see [`declared_defaults`]).
#[derive(Debug, Clone, PartialEq)]
pub struct RibbonParams {
    pub spacing_m: f64,
    pub chunk_m: f64,
    pub cross_samples: usize,
    pub lift_m: f64,
    pub shoulder_m: f64,
    pub shoulder_max_frac: f64,
    pub texture_length_m: f64,
}

impl Default for RibbonParams {
    fn default() -> Self {
        Self::from_declared(&declared_defaults()).expect("defaults are in range")
    }
}

impl RibbonParams {
    /// Values from a declared list, each checked against its range.
    pub fn from_declared(d: &[Declared]) -> Result<Self> {
        let get = |name: &str| -> Result<f64> {
            let p = d
                .iter()
                .find(|p| p.name == name)
                .ok_or_else(|| Error::Invalid(format!("ribbon value {name} not declared")))?;
            if p.v.is_finite() && p.v >= p.min && p.v <= p.max {
                Ok(p.v)
            } else {
                Err(Error::Invalid(format!("{name} = {} outside [{}, {}]", p.v, p.min, p.max)))
            }
        };
        Ok(Self {
            spacing_m: get("spacing_m")?,
            chunk_m: get("chunk_m")?,
            cross_samples: get("cross_samples")?.round() as usize,
            lift_m: get("lift_m")?,
            shoulder_m: get("shoulder_m")?,
            shoulder_max_frac: get("shoulder_max_frac")?,
            texture_length_m: get("texture_length_m")?,
        })
    }
}

/// A smoothed centreline: interpolating centripetal Catmull-Rom through the input points, with an arc-length table.
#[derive(Debug, Clone)]
pub struct Centreline {
    /// Distinct input points.
    pts: Vec<[f64; 2]>,
    /// Per piece `i` (from `pts[i]` to `pts[i + 1]`): Hermite end tangents, scaled to the unit parameter.
    tangents: Vec<([f64; 2], [f64; 2])>,
    /// Dense table: (arc length, piece, parameter), increasing in arc length.
    table: Vec<(f64, usize, f64)>,
}

fn sub(a: [f64; 2], b: [f64; 2]) -> [f64; 2] {
    [a[0] - b[0], a[1] - b[1]]
}

fn dist(a: [f64; 2], b: [f64; 2]) -> f64 {
    let d = sub(a, b);
    d[0].hypot(d[1])
}

impl Centreline {
    /// The spline through `points` (consecutive duplicates dropped); `None` with fewer than two distinct points.
    /// `spacing_m` sets the density of the arc-length table.
    pub fn new(points: &[[f64; 2]], spacing_m: f64) -> Option<Self> {
        let mut pts: Vec<[f64; 2]> = Vec::with_capacity(points.len());
        for &p in points {
            if pts.last().is_none_or(|&q| dist(p, q) > DUP_EPS_M) {
                pts.push(p);
            }
        }
        if pts.len() < 2 {
            return None;
        }
        let n = pts.len();
        // Phantom end points reflected through the ends.
        let at = |i: isize| -> [f64; 2] {
            if i < 0 {
                let (a, b) = (pts[0], pts[1]);
                [2.0 * a[0] - b[0], 2.0 * a[1] - b[1]]
            } else if i as usize >= n {
                let (a, b) = (pts[n - 1], pts[n - 2]);
                [2.0 * a[0] - b[0], 2.0 * a[1] - b[1]]
            } else {
                pts[i as usize]
            }
        };
        let mut tangents = Vec::with_capacity(n - 1);
        for i in 0..n - 1 {
            let i = i as isize;
            let (p0, p1, p2, p3) = (at(i - 1), at(i), at(i + 1), at(i + 2));
            // Centripetal knot intervals (alpha = 0.5); phantom points are distinct from their neighbours.
            let d01 = dist(p0, p1).sqrt().max(DUP_EPS_M);
            let d12 = dist(p1, p2).sqrt().max(DUP_EPS_M);
            let d23 = dist(p2, p3).sqrt().max(DUP_EPS_M);
            // Non-uniform Catmull-Rom tangents at p1 and p2 (per knot unit), then scaled to the piece's interval.
            let m1 = |k: usize| -> f64 {
                let (a, b, c) = (p0[k], p1[k], p2[k]);
                (b - a) / d01 - (c - a) / (d01 + d12) + (c - b) / d12
            };
            let m2 = |k: usize| -> f64 {
                let (b, c, d) = (p1[k], p2[k], p3[k]);
                (c - b) / d12 - (d - b) / (d12 + d23) + (d - c) / d23
            };
            tangents.push(([m1(0) * d12, m1(1) * d12], [m2(0) * d12, m2(1) * d12]));
        }
        let mut c = Self { pts, tangents, table: Vec::new() };
        c.build_table(spacing_m);
        Some(c)
    }

    fn build_table(&mut self, spacing_m: f64) {
        let mut s = 0.0;
        let mut table = vec![(0.0, 0, 0.0)];
        for i in 0..self.pts.len() - 1 {
            let chord = dist(self.pts[i], self.pts[i + 1]);
            let steps = ((chord / spacing_m * ARC_SUBSTEPS_PER_SPACING).ceil() as usize).max(4);
            let mut prev = self.pts[i];
            for k in 1..=steps {
                let t = k as f64 / steps as f64;
                let p = self.piece(i, t).0;
                s += dist(p, prev);
                prev = p;
                table.push((s, i, t));
            }
        }
        self.table = table;
    }

    /// Position and derivative (per unit parameter) of piece `i` at `t` in `[0, 1]`.
    fn piece(&self, i: usize, t: f64) -> ([f64; 2], [f64; 2]) {
        let (p1, p2) = (self.pts[i], self.pts[i + 1]);
        let (m1, m2) = self.tangents[i];
        let (t2, t3) = (t * t, t * t * t);
        let h00 = 2.0 * t3 - 3.0 * t2 + 1.0;
        let h10 = t3 - 2.0 * t2 + t;
        let h01 = -2.0 * t3 + 3.0 * t2;
        let h11 = t3 - t2;
        let d00 = 6.0 * t2 - 6.0 * t;
        let d10 = 3.0 * t2 - 4.0 * t + 1.0;
        let d01 = -6.0 * t2 + 6.0 * t;
        let d11 = 3.0 * t2 - 2.0 * t;
        let f = |k: usize| h00 * p1[k] + h10 * m1[k] + h01 * p2[k] + h11 * m2[k];
        let g = |k: usize| d00 * p1[k] + d10 * m1[k] + d01 * p2[k] + d11 * m2[k];
        ([f(0), f(1)], [g(0), g(1)])
    }

    /// Arc length of the whole centreline, metres.
    pub fn length(&self) -> f64 {
        self.table.last().map_or(0.0, |e| e.0)
    }

    /// The distinct input points the spline passes through.
    pub fn knots(&self) -> &[[f64; 2]] {
        &self.pts
    }

    /// Position and unit tangent at arc length `s` (clamped to `[0, length]`).
    pub fn eval(&self, s: f64) -> ([f64; 2], [f64; 2]) {
        let s = s.clamp(0.0, self.length());
        let k = self.table.partition_point(|e| e.0 < s).clamp(1, self.table.len() - 1);
        let (sa, ia, ta) = self.table[k - 1];
        let (sb, ib, tb) = self.table[k];
        let (i, t) = if ia == ib {
            let f = if sb > sa { (s - sa) / (sb - sa) } else { 0.0 };
            (ia, ta + (tb - ta) * f)
        } else {
            // A piece boundary: `ta` is 1 of piece `ia`, `tb` the first step of piece `ib`.
            let f = if sb > sa { (s - sa) / (sb - sa) } else { 0.0 };
            (ib, tb * f)
        };
        let (p, d) = self.piece(i, t);
        let len = d[0].hypot(d[1]);
        let tan = if len > DUP_EPS_M {
            [d[0] / len, d[1] / len]
        } else {
            let c = sub(self.pts[i + 1], self.pts[i]);
            let l = c[0].hypot(c[1]);
            [c[0] / l, c[1] / l]
        };
        (p, tan)
    }

    /// Arc length of the table point nearest to `p` (accurate to the table density, refined on the nearest
    /// table segment).
    pub fn project(&self, p: [f64; 2]) -> f64 {
        let mut best = (f64::INFINITY, 0.0);
        let mut prev: Option<([f64; 2], f64)> = None;
        for &(s, i, t) in &self.table {
            let q = self.piece(i, t).0;
            if let Some((a, sa)) = prev {
                let ab = sub(q, a);
                let l2 = ab[0] * ab[0] + ab[1] * ab[1];
                let f = if l2 > 0.0 {
                    (((p[0] - a[0]) * ab[0] + (p[1] - a[1]) * ab[1]) / l2).clamp(0.0, 1.0)
                } else {
                    0.0
                };
                let c = [a[0] + ab[0] * f, a[1] + ab[1] * f];
                let dd = dist(p, c);
                if dd < best.0 {
                    best = (dd, sa + (s - sa) * f);
                }
            }
            prev = Some((q, s));
        }
        best.1
    }
}

/// A stretch of a segment the ribbon skips (a bridge or causeway; decks are a later slice).
#[derive(Debug, Clone, PartialEq)]
pub struct Span {
    pub kind: CrossingKind,
    /// The crossing's own guide-frame polyline.
    pub points: Vec<[f64; 2]>,
    /// Arc-length interval along the smoothed centreline, metres (merged spans keep the first kind).
    pub s0_m: f64,
    pub s1_m: f64,
}

/// One cullable piece of ribbon mesh data.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct RibbonChunk {
    /// Guide-frame metres (x east, y north, z up) the positions are relative to: the chunk's first centreline
    /// point on the height function (no lift).
    pub origin: [f64; 3],
    /// Arc-length interval of the chunk along the centreline, metres.
    pub s0_m: f64,
    pub s1_m: f64,
    /// Rows along, each `cross_samples` vertices wide.
    pub rows: usize,
    pub cols: usize,
    /// Relative to `origin`, row-major (row = along).
    pub positions: Vec<[f32; 3]>,
    /// Unit, z up (guide frame).
    pub normals: Vec<[f32; 3]>,
    /// `u` across 0 (left) .. 1 (right), `v` = metres from the segment start / `texture_length_m`.
    pub uvs: Vec<[f32; 2]>,
    /// 0 on the edges, 1 from the shoulder lines inward.
    pub fade: Vec<f32>,
    /// Triangle list, counter-clockwise seen from +z.
    pub indices: Vec<u32>,
}

/// Ribbon mesh data of one segment.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Ribbon {
    pub chunks: Vec<RibbonChunk>,
    pub spans: Vec<Span>,
    /// Arc length of the smoothed centreline, metres.
    pub length_m: f64,
}

/// Cross-section offsets from the centreline (positive = left), left edge first, and their fade.
fn cross_section(width_m: f64, p: &RibbonParams) -> Vec<(f64, f32)> {
    let n = p.cross_samples.max(3);
    let half = width_m * 0.5;
    let shoulder = p.shoulder_m.min(p.shoulder_max_frac * width_m).min(half);
    let inner = half - shoulder;
    let mut cols = Vec::with_capacity(n);
    cols.push((half, 0.0));
    let k = n - 2;
    for j in 0..k {
        let off = if k == 1 { 0.0 } else { inner - 2.0 * inner * j as f64 / (k - 1) as f64 };
        cols.push((off, 1.0));
    }
    cols.push((-half, 0.0));
    cols
}

/// Ground runs of `[0, len]` outside the deck spans, as arc-length intervals.
fn ground_runs(len: f64, spans: &[Span]) -> Vec<(f64, f64)> {
    let mut runs = Vec::new();
    let mut s = 0.0;
    for sp in spans {
        if sp.s0_m > s {
            runs.push((s, sp.s0_m));
        }
        s = s.max(sp.s1_m);
    }
    if len > s {
        runs.push((s, len));
    }
    runs.retain(|&(a, b)| b - a > DUP_EPS_M);
    runs
}

/// Deck spans of a segment on its centreline, sorted and merged.
fn deck_spans(seg: &Segment, line: &Centreline) -> Vec<Span> {
    let mut spans: Vec<Span> = seg
        .crossings
        .iter()
        .filter(|c| c.kind.is_deck() && !c.points.is_empty())
        .map(|c| {
            let a = line.project(c.points[0]);
            let b = line.project(c.points[c.points.len() - 1]);
            Span { kind: c.kind, points: c.points.clone(), s0_m: a.min(b), s1_m: a.max(b) }
        })
        .collect();
    spans.sort_by(|a, b| a.s0_m.total_cmp(&b.s0_m).then(a.s1_m.total_cmp(&b.s1_m)));
    let mut merged: Vec<Span> = Vec::with_capacity(spans.len());
    for sp in spans {
        match merged.last_mut() {
            Some(last) if sp.s0_m <= last.s1_m => {
                last.s1_m = last.s1_m.max(sp.s1_m);
                last.points.extend(sp.points);
            }
            _ => merged.push(sp),
        }
    }
    merged
}

fn normalize3(v: [f64; 3]) -> [f64; 3] {
    let l = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
    if l > 0.0 { [v[0] / l, v[1] / l, v[2] / l] } else { [0.0, 0.0, 1.0] }
}

/// Ribbon mesh data of `seg`, draped on `height` (guide frame: `height(x_east, y_north)` in metres).
pub fn ribbon(seg: &Segment, params: &RibbonParams, height: &dyn Fn(f64, f64) -> f64) -> Ribbon {
    let Some(line) = Centreline::new(&seg.points, params.spacing_m) else {
        return Ribbon::default();
    };
    let len = line.length();
    let spans = deck_spans(seg, &line);
    let cols = cross_section(seg.width_m, params);
    let nc = cols.len();
    let mut chunks = Vec::new();
    for (r0, r1) in ground_runs(len, &spans) {
        let steps = ((r1 - r0) / params.spacing_m).ceil().max(1.0) as usize;
        let ds = (r1 - r0) / steps as f64;
        let nr = steps + 1;
        // World positions (f64) and arc lengths of the run's rows.
        let mut world: Vec<[f64; 3]> = Vec::with_capacity(nr * nc);
        let mut centre: Vec<[f64; 3]> = Vec::with_capacity(nr);
        let mut srow: Vec<f64> = Vec::with_capacity(nr);
        for r in 0..nr {
            let s = if r == steps { r1 } else { r0 + ds * r as f64 };
            let (c, t) = line.eval(s);
            let left = [-t[1], t[0]];
            for &(off, _) in &cols {
                let (x, y) = (c[0] + left[0] * off, c[1] + left[1] * off);
                world.push([x, y, height(x, y) + params.lift_m]);
            }
            centre.push([c[0], c[1], height(c[0], c[1])]);
            srow.push(s);
        }
        // Normals by central differences on the run's grid (one-sided at its borders).
        let at = |r: usize, j: usize| world[r * nc + j];
        let mut normals: Vec<[f64; 3]> = Vec::with_capacity(nr * nc);
        for r in 0..nr {
            let (ra, rb) = (r.saturating_sub(1), (r + 1).min(nr - 1));
            for j in 0..nc {
                let (ja, jb) = (j.saturating_sub(1), (j + 1).min(nc - 1));
                let (a, b) = (at(ra, j), at(rb, j));
                let along = [b[0] - a[0], b[1] - a[1], b[2] - a[2]];
                // Columns run left to right; `across` points right.
                let (a, b) = (at(r, ja), at(r, jb));
                let across = [b[0] - a[0], b[1] - a[1], b[2] - a[2]];
                let n = [
                    across[1] * along[2] - across[2] * along[1],
                    across[2] * along[0] - across[0] * along[2],
                    across[0] * along[1] - across[1] * along[0],
                ];
                let n = normalize3(n);
                normals.push(if n[2] < 0.0 { [-n[0], -n[1], -n[2]] } else { n });
            }
        }
        // Chunks of at most `chunk_m`, sharing seam rows.
        let per = ((params.chunk_m / ds).floor() as usize).max(1);
        let mut a = 0;
        while a < steps {
            let b = (a + per).min(steps);
            chunks.push(build_chunk(&world, &normals, &centre, &srow, &cols, a, b, params));
            a = b;
        }
    }
    Ribbon { chunks, spans, length_m: len }
}

/// One chunk from rows `a..=b` of a run.
#[allow(clippy::too_many_arguments)]
fn build_chunk(
    world: &[[f64; 3]],
    normals: &[[f64; 3]],
    centre: &[[f64; 3]],
    srow: &[f64],
    cols: &[(f64, f32)],
    a: usize,
    b: usize,
    params: &RibbonParams,
) -> RibbonChunk {
    let nc = cols.len();
    let rows = b - a + 1;
    let origin = centre[a];
    let mut ch = RibbonChunk {
        origin,
        s0_m: srow[a],
        s1_m: srow[b],
        rows,
        cols: nc,
        positions: Vec::with_capacity(rows * nc),
        normals: Vec::with_capacity(rows * nc),
        uvs: Vec::with_capacity(rows * nc),
        fade: Vec::with_capacity(rows * nc),
        indices: Vec::with_capacity((rows - 1) * (nc - 1) * 6),
    };
    let full_w = (cols[0].0 - cols[nc - 1].0).max(DUP_EPS_M);
    for r in a..=b {
        let v = (srow[r] / params.texture_length_m) as f32;
        for (j, &(off, fade)) in cols.iter().enumerate() {
            let w = world[r * nc + j];
            let n = normals[r * nc + j];
            ch.positions.push([(w[0] - origin[0]) as f32, (w[1] - origin[1]) as f32, (w[2] - origin[2]) as f32]);
            ch.normals.push([n[0] as f32, n[1] as f32, n[2] as f32]);
            ch.uvs.push([((cols[0].0 - off) / full_w) as f32, v]);
            ch.fade.push(fade);
        }
    }
    let nc32 = nc as u32;
    for r in 0..(rows - 1) as u32 {
        for j in 0..nc32 - 1 {
            // Column j is left of j + 1, row r behind r + 1: CCW from above.
            let v00 = r * nc32 + j;
            let (v01, v10, v11) = (v00 + 1, v00 + nc32, v00 + nc32 + 1);
            ch.indices.extend_from_slice(&[v00, v01, v11, v00, v11, v10]);
        }
    }
    ch
}
