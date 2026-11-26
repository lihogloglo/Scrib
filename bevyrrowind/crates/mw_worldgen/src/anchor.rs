//! Anchors of the same map at 10× (spec `worldgen-adapter.md` "Same map at 10× (WG-X)", steps 3-4; WG-X2).
//!
//! Human-made references ([`crate::classify`]) cluster by single linkage on the gaps between their discs; clusters
//! whose stamps would overlap **in the scaled world** (weight > 0) merge, until none do. Each cluster is an anchor
//! that moves rigidly, `world = S·c + (p − c)`, `height + L`, and carries a stamp over its footprint plus a band:
//! the source heights + L, weight 1 on the footprint and a C1 smootherstep to 0 across the band. The band scales
//! with the anchor (`clamp(k · R, min, max)`, R the footprint radius). **Point anchors** (few members or a small
//! footprint: a lone signpost, lamp or crate) get no stamp: their members move by the same rigid offset and are
//! grounded on the final ground (each keeps its offset above the source ground), so the stamp budget (Σ w·h, the
//! GPU stamp field) goes to settlements.
//!
//! **Lift L**: the median over the band ring of (final scaled height − source height) on samples dry in both, so
//! the stamp meets the land around it; 0 when a member stands on sea-connected water (docks, wrecks, Vivec).

// Grid maths: sample counts stay far below 2^31 and heights within f32 range (as in sf_worldgen).
#![allow(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_possible_wrap,
    clippy::many_single_char_names
)]

use std::collections::{BTreeMap, VecDeque};

use sf_worldgen::anchors::{AnchorPlace, Footprint};
use sf_worldgen::filter::Declared;
use sf_worldgen::stamps::Stamp;

/// Sea level of the source and the scaled guide, metres.
// tunables: const (format fact: Morrowind's exterior water plane is at z = 0; the scaled guide keeps it)
pub const SEA_LEVEL_M: f64 = 0.0;

/// The anchor values (spec `tunables.md` "Definition": value, unit, range, source).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Params {
    /// Single-linkage gap between two human-made references' discs that joins them, metres.
    pub link_gap_m: f64,
    /// Disc radius of a reference whose model has no bounds, metres.
    pub default_radius_m: f64,
    /// Upper bound of a reference's disc radius, metres.
    pub max_radius_m: f64,
    /// Added to every member disc to make the footprint, metres.
    pub margin_m: f64,
    /// Band width per metre of footprint radius, and its bounds (source scale), metres.
    pub band_k: f64,
    pub band_min_m: f64,
    pub band_max_m: f64,
    /// Anchors of at most this many members, or with a footprint radius under `point_max_radius_m`, get no stamp.
    pub point_max_members: usize,
    pub point_max_radius_m: f64,
    /// Upper bound of the total stamp samples (Σ w·h; the GPU stamp field's budget).
    pub stamp_samples_max: usize,
    /// Rocks whose largest bounds edge reaches this are terrain (nature), metres.
    pub rock_min_m: f64,
    /// Water narrower than twice this is not open sea (rivers, canals), metres.
    pub open_water_radius_m: f64,
}

/// The declared anchor values.
pub fn declared() -> Vec<Declared> {
    let d = |name, v, unit, min, max, help| Declared { name, v, unit, min, max, source: "Guess".into(), help };
    vec![
        d(
            "anchor.link_gap_m",
            30.0,
            "m",
            0.0,
            200.0,
            "two human-made references whose discs are at most this far apart join one anchor (single linkage); \
             a settlement must stay one anchor or its houses spread S times apart",
        ),
        d("anchor.default_radius_m", 2.0, "m", 0.1, 50.0, "disc radius of a reference whose model has no bounds"),
        d(
            "anchor.max_radius_m",
            150.0,
            "m",
            1.0,
            1000.0,
            "upper bound of a reference's disc radius (the largest Morrowind structures, Vivec's cantons, are ~100 m)",
        ),
        d("anchor.margin_m", 6.0, "m", 0.0, 50.0, "added to each member disc: the footprint keeps a ground apron"),
        d("anchor.band_k", 0.5, "ratio", 0.0, 4.0, "blend band width per metre of footprint radius (source scale)"),
        d("anchor.band_min_m", 30.0, "m", 1.0, 500.0, "narrowest blend band (smootherstep from weight 1 to 0)"),
        d("anchor.band_max_m", 150.0, "m", 1.0, 1000.0, "widest blend band (a town; brief Guess 100-150 m)"),
        d(
            "anchor.point_max_members",
            2.0,
            "count",
            0.0,
            100.0,
            "anchors of at most this many members are point anchors: no stamp, members grounded on the final ground",
        ),
        d(
            "anchor.point_max_radius_m",
            12.0,
            "m",
            0.0,
            200.0,
            "anchors whose footprint radius (margin included) is under this are point anchors (a lamp, a signpost)",
        ),
        Declared {
            name: "anchor.stamp_samples_max",
            v: 8.0e6,
            unit: "count",
            min: 1.0e5,
            max: 3.2e7,
            source: "Reference(sf_worldgen stamps: one 128 MiB GPU storage binding holds the 39 MiB x10 guide; 8 M \
                     samples x 8 B = 64 MiB)"
                .into(),
            help: "upper bound of the total stamp samples (sum of w x h); worldgen-x10 fails above it",
        },
        d(
            "anchor.open_water_radius_m",
            40.0,
            "m",
            1.0,
            500.0,
            "the open sea is the sea-connected water opened by this radius: a member on it (dock, wreck) keeps L = 0;              rivers and channels narrower than twice this (Balmora's Odai) don't count",
        ),
        Declared {
            name: "anchor.rock_min_m",
            v: 14.0,
            unit: "m",
            min: 1.0,
            max: 100.0,
            source: "Reference(worldgen-adapter.md: rocks of ~14 m and more fake terrain)".into(),
            help: "a rock whose largest bounds edge reaches this is terrain (nature); smaller ones are 'other'",
        },
    ]
}

impl Default for Params {
    fn default() -> Self {
        let v = |name: &str| declared().into_iter().find(|d| d.name == name).map_or(f64::NAN, |d| d.v);
        Self {
            link_gap_m: v("anchor.link_gap_m"),
            default_radius_m: v("anchor.default_radius_m"),
            max_radius_m: v("anchor.max_radius_m"),
            margin_m: v("anchor.margin_m"),
            band_k: v("anchor.band_k"),
            band_min_m: v("anchor.band_min_m"),
            band_max_m: v("anchor.band_max_m"),
            point_max_members: v("anchor.point_max_members") as usize,
            point_max_radius_m: v("anchor.point_max_radius_m"),
            stamp_samples_max: v("anchor.stamp_samples_max") as usize,
            rock_min_m: v("anchor.rock_min_m"),
            open_water_radius_m: v("anchor.open_water_radius_m"),
        }
    }
}

impl Params {
    /// Whether an anchor of `members` with footprint radius `radius_m` is a point anchor (no stamp).
    pub fn is_point(&self, members: usize, radius_m: f64) -> bool {
        members <= self.point_max_members || radius_m < self.point_max_radius_m
    }

    /// The blend band of such an anchor, metres; 0 for a point anchor.
    pub fn band_of(&self, members: usize, radius_m: f64) -> f64 {
        if self.is_point(members, radius_m) {
            0.0
        } else {
            (self.band_k * radius_m).clamp(self.band_min_m, self.band_max_m)
        }
    }
}

/// A height field on a regular grid, row 0 north, column 0 west.
#[derive(Clone, Copy, Debug)]
pub struct Field<'a> {
    pub w: usize,
    pub h: usize,
    /// `[x east, y north]` of sample (0, 0), metres.
    pub origin_m: [f64; 2],
    pub spacing_m: f64,
    pub data: &'a [f32],
}

impl Field<'_> {
    /// Fractional `(col, row)` of a point.
    fn grid(&self, x: f64, y: f64) -> (f64, f64) {
        ((x - self.origin_m[0]) / self.spacing_m, (self.origin_m[1] - y) / self.spacing_m)
    }

    fn at(&self, c: usize, r: usize) -> f64 {
        f64::from(self.data[r * self.w + c])
    }

    /// Bilinear height at a point, clamped to the grid.
    pub fn bilinear(&self, x: f64, y: f64) -> f64 {
        let (u, v) = self.grid(x, y);
        let u = u.clamp(0.0, (self.w - 1) as f64);
        let v = v.clamp(0.0, (self.h - 1) as f64);
        let (c0, r0) = (u.floor() as usize, v.floor() as usize);
        let (c1, r1) = ((c0 + 1).min(self.w - 1), (r0 + 1).min(self.h - 1));
        let (fu, fv) = (u - c0 as f64, v - r0 as f64);
        let top = self.at(c0, r0) * (1.0 - fu) + self.at(c1, r0) * fu;
        let bottom = self.at(c0, r1) * (1.0 - fu) + self.at(c1, r1) * fu;
        top * (1.0 - fv) + bottom * fv
    }

    /// Index of the sample nearest a point (clamped).
    pub fn nearest(&self, x: f64, y: f64) -> usize {
        let (u, v) = self.grid(x, y);
        let c = u.round().clamp(0.0, (self.w - 1) as f64) as usize;
        let r = v.round().clamp(0.0, (self.h - 1) as f64) as usize;
        r * self.w + c
    }
}

/// Samples below sea level 4-connected to the grid's border.
pub fn sea_connected(f: &Field<'_>) -> Vec<bool> {
    let wet: Vec<bool> = f.data.iter().map(|&h| f64::from(h) < SEA_LEVEL_M).collect();
    border_connected(&wet, f.w, f.h)
}

/// The open sea: sea-connected water opened by `radius_m` (a morphological opening of the wet mask, so rivers and
/// channels narrower than `2 · radius_m` drop out), each wet sample within `radius_m` of an opened core that
/// reaches the grid's border.
pub fn open_sea(f: &Field<'_>, radius_m: f64) -> Vec<bool> {
    let (w, h) = (f.w, f.h);
    let wet: Vec<bool> = f.data.iter().map(|&v| f64::from(v) < SEA_LEVEL_M).collect();
    let dry: Vec<bool> = wet.iter().map(|&x| !x).collect();
    let r = radius_m / f.spacing_m;
    let core: Vec<bool> = distance_samples(&dry, w, h).into_iter().zip(&wet).map(|(d, &x)| x && d > r).collect();
    let core = border_connected(&core, w, h);
    distance_samples(&core, w, h).into_iter().zip(&wet).map(|(d, &x)| x && d <= r).collect()
}

/// The `mask` samples 4-connected to the grid's border through `mask` samples.
fn border_connected(mask: &[bool], w: usize, h: usize) -> Vec<bool> {
    let mut seen = vec![false; w * h];
    let mut queue = VecDeque::new();
    for k in (0..w).chain((h - 1) * w..h * w).chain((0..h).map(|r| r * w)).chain((0..h).map(|r| r * w + w - 1)) {
        if mask[k] && !seen[k] {
            seen[k] = true;
            queue.push_back(k);
        }
    }
    while let Some(k) = queue.pop_front() {
        let (c, r) = (k % w, k / w);
        let mut push = |n: usize| {
            if !seen[n] && mask[n] {
                seen[n] = true;
                queue.push_back(n);
            }
        };
        if c > 0 {
            push(k - 1);
        }
        if c + 1 < w {
            push(k + 1);
        }
        if r > 0 {
            push(k - w);
        }
        if r + 1 < h {
            push(k + w);
        }
    }
    seen
}

/// One human-made reference as an anchor member.
#[derive(Clone, Debug, PartialEq)]
pub struct Member {
    pub key: String,
    /// Source metres, x east, y north.
    pub xy: [f64; 2],
    /// Disc radius without the margin, metres.
    pub r: f64,
    /// Stands on the open sea ([`open_sea`]).
    pub on_sea: bool,
    /// The census group (`rule: model folder`).
    pub group: String,
}

struct UnionFind(Vec<usize>);

impl UnionFind {
    fn new(n: usize) -> Self {
        Self((0..n).collect())
    }
    fn find(&mut self, mut i: usize) -> usize {
        while self.0[i] != i {
            self.0[i] = self.0[self.0[i]];
            i = self.0[i];
        }
        i
    }
    /// Joins two sets; the lower root wins (deterministic). True when they were apart.
    fn union(&mut self, a: usize, b: usize) -> bool {
        let (a, b) = (self.find(a), self.find(b));
        if a == b {
            return false;
        }
        let (lo, hi) = (a.min(b), a.max(b));
        self.0[hi] = lo;
        true
    }
    /// The sets, each sorted, ordered by their smallest element.
    fn groups(&mut self) -> Vec<Vec<usize>> {
        let mut by_root: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
        for i in 0..self.0.len() {
            let r = self.find(i);
            by_root.entry(r).or_default().push(i);
        }
        let mut groups: Vec<Vec<usize>> = by_root.into_values().collect();
        groups.sort_by_key(|g| g[0]);
        groups
    }
}

/// `[min_x, min_y, max_x, max_y]` of discs `(centre, radius)`.
fn disc_bounds(discs: impl Iterator<Item = ([f64; 2], f64)>) -> [f64; 4] {
    discs.fold([f64::INFINITY, f64::INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY], |b, (p, r)| {
        [b[0].min(p[0] - r), b[1].min(p[1] - r), b[2].max(p[0] + r), b[3].max(p[1] + r)]
    })
}

/// An anchor's centre and footprint radius: the centre of its footprint's bounds (member discs + margin) and half
/// their larger edge.
pub fn centre(members: &[&Member], p: &Params) -> ([f64; 2], f64) {
    let b = disc_bounds(members.iter().map(|m| (m.xy, m.r.min(p.max_radius_m) + p.margin_m)));
    ([f64::midpoint(b[0], b[2]), f64::midpoint(b[1], b[3])], 0.5 * (b[2] - b[0]).max(b[3] - b[1]))
}

/// Clusters `members` into anchors (module docs). Returns member indices per anchor, each sorted by key, the
/// anchors ordered by their smallest key: the result doesn't depend on the input order. `guard_m` widens the
/// no-overlap test (one stamp sample covers the raster's slack).
pub fn cluster(members: &[Member], p: &Params, scale: f64, guard_m: f64) -> Vec<Vec<usize>> {
    let n = members.len();
    // Work in key order (ranks); map back at the end.
    let mut order: Vec<usize> = (0..n).collect();
    order.sort_by(|&a, &b| members[a].key.cmp(&members[b].key).then(a.cmp(&b)));
    let m: Vec<&Member> = order.iter().map(|&i| &members[i]).collect();
    let r: Vec<f64> = m.iter().map(|x| x.r.min(p.max_radius_m)).collect();
    let r_max = r.iter().copied().fold(0.0, f64::max);

    // 1. Single linkage on disc gaps, through a uniform bucket grid.
    let bucket = 64.0_f64;
    let key = |xy: [f64; 2]| ((xy[0] / bucket).floor() as i64, (xy[1] / bucket).floor() as i64);
    let mut grid: BTreeMap<(i64, i64), Vec<usize>> = BTreeMap::new();
    for (i, x) in m.iter().enumerate() {
        grid.entry(key(x.xy)).or_default().push(i);
    }
    let mut uf = UnionFind::new(n);
    for i in 0..n {
        let reach = r[i] + p.link_gap_m + r_max;
        let span = (reach / bucket).ceil() as i64;
        let (bx, by) = key(m[i].xy);
        for gx in bx - span..=bx + span {
            for gy in by - span..=by + span {
                for &j in grid.get(&(gx, gy)).into_iter().flatten() {
                    if j > i {
                        let d = (m[i].xy[0] - m[j].xy[0]).hypot(m[i].xy[1] - m[j].xy[1]);
                        if d - r[i] - r[j] <= p.link_gap_m {
                            uf.union(i, j);
                        }
                    }
                }
            }
        }
    }

    // 2. Merge clusters whose stamps (footprint + band) would overlap in the scaled world, until none do.
    loop {
        let groups = uf.groups();
        // Per group: the scaled-world offset (S − 1)·c and how far its stamp reaches past the footprint (its band,
        // 0 for a point anchor, plus the guard).
        let shape: Vec<([f64; 2], f64)> = groups
            .iter()
            .map(|g| {
                let ms: Vec<&Member> = g.iter().map(|&i| m[i]).collect();
                let (c, radius) = centre(&ms, p);
                ([(scale - 1.0) * c[0], (scale - 1.0) * c[1]], p.band_of(g.len(), radius) + guard_m)
            })
            .collect();
        let boxes: Vec<[f64; 4]> = groups
            .iter()
            .zip(&shape)
            .map(|(g, &(o, e))| {
                let b = disc_bounds(g.iter().map(|&i| ([m[i].xy[0] + o[0], m[i].xy[1] + o[1]], r[i] + p.margin_m)));
                [b[0] - e, b[1] - e, b[2] + e, b[3] + e]
            })
            .collect();
        let mut by_x: Vec<usize> = (0..groups.len()).collect();
        by_x.sort_by(|&a, &b| boxes[a][0].total_cmp(&boxes[b][0]).then(a.cmp(&b)));
        let mut merged = false;
        for (k, &a) in by_x.iter().enumerate() {
            for &b in &by_x[k + 1..] {
                if boxes[b][0] > boxes[a][2] {
                    break;
                }
                if boxes[b][1] > boxes[a][3] || boxes[b][3] < boxes[a][1] {
                    continue;
                }
                let ((oa, ea), (ob, eb)) = (shape[a], shape[b]);
                let touch = groups[a].iter().any(|&i| {
                    groups[b].iter().any(|&j| {
                        let dx = (m[i].xy[0] + oa[0]) - (m[j].xy[0] + ob[0]);
                        let dy = (m[i].xy[1] + oa[1]) - (m[j].xy[1] + ob[1]);
                        dx.hypot(dy) < r[i] + r[j] + 2.0 * p.margin_m + ea + eb
                    })
                });
                if touch {
                    merged |= uf.union(groups[a][0], groups[b][0]);
                }
            }
        }
        if !merged {
            return groups.into_iter().map(|g| g.into_iter().map(|i| order[i]).collect()).collect();
        }
    }
}

/// C1 (and C2) smootherstep on `[0, 1]`: `t³(6t² − 15t + 10)`.
pub fn smootherstep(t: f64) -> f64 {
    let t = t.clamp(0.0, 1.0);
    t * t * t * (t * (6.0 * t - 15.0) + 10.0)
}

/// Exact squared Euclidean distance transform in one dimension (Felzenszwalb & Huttenlocher 2012).
fn edt_1d(f: &[f64], out: &mut [f64], v: &mut [usize], z: &mut [f64]) {
    let n = f.len();
    let mut k = 0usize;
    v[0] = 0;
    z[0] = f64::NEG_INFINITY;
    z[1] = f64::INFINITY;
    for q in 1..n {
        let mut s;
        loop {
            let p = v[k];
            s = ((f[q] + (q * q) as f64) - (f[p] + (p * p) as f64)) / (2.0 * (q as f64 - p as f64));
            // z[0] = -inf, so k never goes below 0.
            if s <= z[k] {
                k -= 1;
            } else {
                break;
            }
        }
        k += 1;
        v[k] = q;
        z[k] = s;
        z[k + 1] = f64::INFINITY;
    }
    k = 0;
    for (q, o) in out.iter_mut().enumerate() {
        while z[k + 1] < q as f64 {
            k += 1;
        }
        let p = v[k];
        let d = q as f64 - p as f64;
        *o = d * d + f[p];
    }
}

/// Distance in samples from each sample to the nearest `inside` one (exact Euclidean; large where none).
pub fn distance_samples(inside: &[bool], w: usize, h: usize) -> Vec<f64> {
    const FAR: f64 = 1e20; // tunables: const (sentinel: larger than any squared grid distance)
    let mut g: Vec<f64> = inside.iter().map(|&i| if i { 0.0 } else { FAR }).collect();
    let n = w.max(h);
    let (mut f, mut out, mut v, mut z) = (vec![0.0; n], vec![0.0; n], vec![0usize; n], vec![0.0; n + 1]);
    for c in 0..w {
        for r in 0..h {
            f[r] = g[r * w + c];
        }
        edt_1d(&f[..h], &mut out[..h], &mut v, &mut z);
        for r in 0..h {
            g[r * w + c] = out[r];
        }
    }
    for r in 0..h {
        f[..w].copy_from_slice(&g[r * w..(r + 1) * w]);
        edt_1d(&f[..w], &mut out[..w], &mut v, &mut z);
        g[r * w..(r + 1) * w].copy_from_slice(&out[..w]);
    }
    g.into_iter().map(f64::sqrt).collect()
}

/// Samples of a `w × h` grid (sample (0, 0) at `origin`, row 0 north) inside any disc `[x, y, r]`.
fn raster(discs: &[[f64; 3]], origin: [f64; 2], sp: f64, w: usize, h: usize) -> Vec<bool> {
    let [x0, y0] = origin;
    let mut inside = vec![false; w * h];
    for d in discs {
        let c0 = ((d[0] - d[2] - x0) / sp).floor().max(0.0) as usize;
        let c1 = (((d[0] + d[2] - x0) / sp).ceil() as usize).min(w - 1);
        let r0 = ((y0 - (d[1] + d[2])) / sp).floor().max(0.0) as usize;
        let r1 = (((y0 - (d[1] - d[2])) / sp).ceil() as usize).min(h - 1);
        for r in r0..=r1 {
            for col in c0..=c1 {
                let q = [x0 + col as f64 * sp, y0 - r as f64 * sp];
                if (q[0] - d[0]).hypot(q[1] - d[1]) <= d[2] {
                    inside[r * w + col] = true;
                }
            }
        }
    }
    inside
}

/// Median of a sorted slice (0 when empty).
fn median(sorted: &[f64]) -> f64 {
    let n = sorted.len();
    match n {
        0 => 0.0,
        _ if n % 2 == 1 => sorted[n / 2],
        _ => f64::midpoint(sorted[n / 2 - 1], sorted[n / 2]),
    }
}

/// What one anchor measured.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct AnchorStats {
    pub members: usize,
    /// No stamp: members grounded on the final ground.
    pub point: bool,
    /// Blend band, metres (0 for a point anchor).
    pub band_m: f64,
    /// Stamp samples (w·h), 0 for a point anchor.
    pub stamp_samples: usize,
    /// Footprint samples (weight 1) and their area, m².
    pub footprint_samples: usize,
    pub footprint_m2: f64,
    /// Largest edge of the footprint's bounds, metres.
    pub extent_m: f64,
    pub lift_m: f64,
    /// A member stands on sea-connected water (L = 0).
    pub on_sea: bool,
    /// Dry band samples the lift was measured on.
    pub ring_samples: usize,
    /// p95 of |final − (source + L)| over the dry band, metres.
    pub ring_p95_m: f64,
    /// Footprint samples dry in the source and wet in the stamp, and the reverse.
    pub new_wet: usize,
    pub new_dry: usize,
}

/// One anchor, built: its place in the scaled world, its stamp (none for a point anchor), its numbers.
#[derive(Clone, Debug)]
pub struct Built {
    pub place: AnchorPlace,
    pub stamp: Option<Stamp>,
    pub stats: AnchorStats,
}

fn p95(sorted: &[f64]) -> f64 {
    if sorted.is_empty() { 0.0 } else { sorted[((sorted.len() as f64 * 0.95).ceil() as usize).clamp(1, sorted.len()) - 1] }
}

/// Builds one anchor: a stamp over its footprint + band at the source spacing, or none for a point anchor (its lift
/// is then the ground difference at the centre, a fallback: the members are grounded on the final ground). `src`:
/// the source heights (source frame); `fin`: the final scaled heights (scaled frame) the lift is measured against.
/// `place.stamp` is left `None`; the caller numbers the stamps.
pub fn build_anchor(name: String, members: &[&Member], src: &Field<'_>, fin: &Field<'_>, scale: f64, p: &Params) -> Built {
    let sp = src.spacing_m;
    let discs: Vec<[f64; 3]> = members.iter().map(|m| [m.xy[0], m.xy[1], m.r.min(p.max_radius_m) + p.margin_m]).collect();
    let (c, radius) = centre(members, p);
    let off = [(scale - 1.0) * c[0], (scale - 1.0) * c[1]];
    let on_sea = members.iter().any(|m| m.on_sea);
    let band = p.band_of(members.len(), radius);
    let mut stats = AnchorStats {
        members: members.len(),
        point: p.is_point(members.len(), radius),
        band_m: band,
        extent_m: 2.0 * radius,
        on_sea,
        ..AnchorStats::default()
    };
    let mut place = AnchorPlace {
        name: name.clone(),
        stamp: None,
        source_centre_m: c,
        world_centre_m: [scale * c[0], scale * c[1]],
        lift_m: 0.0,
        members: members.iter().map(|m| m.key.clone()).collect(),
        footprint: Footprint { discs },
    };
    if stats.point {
        let lift = if on_sea { 0.0 } else { fin.bilinear(c[0] + off[0], c[1] + off[1]) - src.bilinear(c[0], c[1]) };
        stats.lift_m = lift;
        place.lift_m = lift;
        return Built { place, stamp: None, stats };
    }

    let [ox, oy] = src.origin_m;
    let b = [c[0] - radius, c[1] - radius, c[0] + radius, c[1] + radius];
    let pad = band + sp;
    let i0 = ((b[0] - pad - ox) / sp).floor() as i64;
    let i1 = ((b[2] + pad - ox) / sp).ceil() as i64;
    let j0 = ((oy - (b[3] + pad)) / sp).floor() as i64;
    let j1 = ((oy - (b[1] - pad)) / sp).ceil() as i64;
    let (w, h) = ((i1 - i0 + 1) as usize, (j1 - j0 + 1) as usize);
    let x0 = ox + i0 as f64 * sp;
    let y0 = oy - j0 as f64 * sp;
    let point = |k: usize| [x0 + (k % w) as f64 * sp, y0 - (k / w) as f64 * sp];

    let inside = raster(&place.footprint.discs, [x0, y0], sp, w, h);
    let weights: Vec<f32> =
        distance_samples(&inside, w, h).iter().map(|&d| (1.0 - smootherstep(d * sp / band)) as f32).collect();
    let source: Vec<f64> = (0..w * h)
        .map(|k| {
            let q = point(k);
            src.bilinear(q[0], q[1])
        })
        .collect();
    let ring: Vec<f64> = (0..w * h)
        .filter(|&k| weights[k] > 0.0 && weights[k] < 1.0)
        .filter_map(|k| {
            let q = point(k);
            let f = fin.bilinear(q[0] + off[0], q[1] + off[1]);
            (source[k] >= SEA_LEVEL_M && f >= SEA_LEVEL_M).then_some(f - source[k])
        })
        .collect();
    let lift = if on_sea {
        0.0
    } else {
        let mut diffs = ring.clone();
        diffs.sort_by(f64::total_cmp);
        median(&diffs)
    };
    let mut mismatch: Vec<f64> = ring.iter().map(|d| (d - lift).abs()).collect();
    mismatch.sort_by(f64::total_cmp);
    stats.lift_m = lift;
    stats.ring_samples = ring.len();
    stats.ring_p95_m = p95(&mismatch);
    stats.stamp_samples = w * h;
    for (k, &wk) in weights.iter().enumerate() {
        if wk >= 1.0 {
            stats.footprint_samples += 1;
            match (source[k] < SEA_LEVEL_M, source[k] + lift < SEA_LEVEL_M) {
                (false, true) => stats.new_wet += 1,
                (true, false) => stats.new_dry += 1,
                _ => {}
            }
        }
    }
    stats.footprint_m2 = stats.footprint_samples as f64 * sp * sp;
    place.lift_m = lift;
    let stamp = Stamp {
        name,
        origin_m: [x0 + off[0], y0 + off[1]],
        dims: [w, h],
        spacing_m: sp,
        heights: source.iter().map(|&s| (s + lift) as f32).collect(),
        weights,
    };
    Built { place, stamp: Some(stamp), stats }
}
