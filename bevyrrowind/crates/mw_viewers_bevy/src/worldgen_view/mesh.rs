//! Terrain meshes for `worldgen-view`, Bevy-free: a guide grid split into square chunks, each built at a few
//! strides (LOD 0 = native spacing) with smooth central-difference normals, slope/altitude vertex colours and
//! skirts that hide the cracks between neighbours at different LODs (the standard chunked-LOD fix, cheaper than
//! stitching: Ulrich, "Rendering Massive Terrains using Chunked Level of Detail Control", SIGGRAPH 2002).
//!
//! Local frame: x east, y up, z south (row 0 = north edge = lowest z), metres; sample (c, r) of a chunk sits at
//! `((c - c0) * spacing, h, (r - r0) * spacing)` relative to the chunk's origin sample `(c0, r0)`.

#![allow(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::many_single_char_names,
    reason = "grid indices and metres: a guide is < 2^31 samples, f32 holds them exactly enough for display"
)]

/// Grid strides of the LOD meshes, finest first: LOD 0 is the guide's native spacing. A chunk's size must be a
/// multiple of the coarsest, so neighbours' edges share sample positions at every LOD.
// tunables: const (mesh layout of the viewer's LOD pyramid; switch distances are the tunables)
pub const LOD_STRIDES: [usize; 3] = [1, 4, 16];

use super::refs::value_noise;

/// A height grid in the viewer's local frame.
#[derive(Debug, Clone)]
pub struct Terrain {
    pub w: usize,
    pub h: usize,
    /// Metres between samples.
    pub spacing: f32,
    pub sea_level: f32,
    /// Highest sample, metres (the colour ramp's summit).
    pub max_h: f32,
    /// Row-major heights, row 0 = north.
    pub heights: Vec<f32>,
    /// Vertex-colour variation: brightness and dry/green shift amplitude (0 = none) and the noise wavelength,
    /// metres ([`colour`]).
    pub tint: Tint,
}

/// Low-frequency vertex-colour variation, so flat ground reads as a surface ([`colour`]).
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Tint {
    /// Amplitude, fraction of the colour (0 = flat colours).
    pub amount: f32,
    /// Wavelength of the coarse octave, metres (the fine octave is a quarter of it).
    pub wavelength_m: f32,
}

impl Terrain {
    pub fn new(w: usize, h: usize, spacing: f32, sea_level: f32, heights: Vec<f32>) -> Self {
        assert_eq!(heights.len(), w * h, "heights must be w * h");
        let max_h = heights.iter().copied().fold(f32::NEG_INFINITY, f32::max);
        Self { w, h, spacing, sea_level, max_h, heights, tint: Tint::default() }
    }

    /// Height of sample (c, r), clamped to the grid.
    pub fn at(&self, c: isize, r: isize) -> f32 {
        let c = c.clamp(0, self.w as isize - 1).cast_unsigned();
        let r = r.clamp(0, self.h as isize - 1).cast_unsigned();
        self.heights[r * self.w + c]
    }

    /// Unit normal at sample (c, r) from central differences `step` samples apart (clamped at the borders).
    pub fn normal(&self, c: usize, r: usize, step: usize) -> [f32; 3] {
        let (c, r, s) = (c.cast_signed(), r.cast_signed(), step.max(1).cast_signed());
        let (c0, c1) = ((c - s).max(0), (c + s).min(self.w as isize - 1));
        let (r0, r1) = ((r - s).max(0), (r + s).min(self.h as isize - 1));
        let dx = (self.at(c1, r) - self.at(c0, r)) / (((c1 - c0).max(1)) as f32 * self.spacing);
        let dz = (self.at(c, r1) - self.at(c, r0)) / (((r1 - r0).max(1)) as f32 * self.spacing);
        normalize([-dx, 1.0, -dz])
    }

    /// Bilinear height at local metres (x east, z south from sample (0, 0)); `None` off the grid.
    pub fn height_at(&self, x: f32, z: f32) -> Option<f32> {
        let (fc, fr) = (x / self.spacing, z / self.spacing);
        if !(fc >= 0.0 && fr >= 0.0 && fc <= (self.w - 1) as f32 && fr <= (self.h - 1) as f32) {
            return None;
        }
        let (c, r) = (fc.floor() as isize, fr.floor() as isize);
        let (tx, tz) = (fc - c as f32, fr - r as f32);
        let top = lerp(self.at(c, r), self.at(c + 1, r), tx);
        let bottom = lerp(self.at(c, r + 1), self.at(c + 1, r + 1), tx);
        Some(lerp(top, bottom, tz))
    }

    /// The highest sample: (column, row, height).
    pub fn peak(&self) -> (usize, usize, f32) {
        let (i, h) = self
            .heights
            .iter()
            .copied()
            .enumerate()
            .fold((0, f32::NEG_INFINITY), |best, (i, h)| if h > best.1 { (i, h) } else { best });
        (i % self.w, i / self.w, h)
    }
}

/// One chunk: samples `c0..=c1` × `r0..=r1` (inclusive, so neighbours share their edge samples).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChunkRect {
    pub c0: usize,
    pub r0: usize,
    pub c1: usize,
    pub r1: usize,
}

/// Splits a `w × h` grid into chunks of `size` quads, west to east, north to south.
pub fn chunks(w: usize, h: usize, size: usize) -> Vec<ChunkRect> {
    let size = size.max(1);
    let mut out = Vec::new();
    let mut r0 = 0;
    while r0 < h - 1 {
        let r1 = (r0 + size).min(h - 1);
        let mut c0 = 0;
        while c0 < w - 1 {
            let c1 = (c0 + size).min(w - 1);
            out.push(ChunkRect { c0, r0, c1, r1 });
            c0 = c1;
        }
        r0 = r1;
    }
    out
}

/// Whether every sample of the chunk is below sea level (the opaque sea hides it, so it isn't built).
pub fn chunk_wet(t: &Terrain, rect: ChunkRect) -> bool {
    (rect.r0..=rect.r1).all(|r| (rect.c0..=rect.c1).all(|c| t.heights[r * t.w + c] < t.sea_level))
}

/// Sample indices `a, a + s, ...` along `a..=b`, always ending on `b`.
pub fn axis(a: usize, b: usize, s: usize) -> Vec<usize> {
    let mut v: Vec<usize> = (a..b).step_by(s.max(1)).collect();
    v.push(b);
    v
}

/// Largest vertical gap, metres, between the chunk's full-resolution edges and the same edges at `stride`
/// (linear between the coarse samples): how far a neighbour at another LOD can be from this one.
pub fn edge_error(t: &Terrain, rect: ChunkRect, stride: usize) -> f32 {
    let mut err = 0.0f32;
    let mut walk = |samples: &dyn Fn(usize) -> f32, a: usize, b: usize| {
        let coarse = axis(a, b, stride);
        for pair in coarse.windows(2) {
            let (i0, i1) = (pair[0], pair[1]);
            let (h0, h1) = (samples(i0), samples(i1));
            for i in i0..=i1 {
                let t = (i - i0) as f32 / (i1 - i0).max(1) as f32;
                err = err.max((samples(i) - lerp(h0, h1, t)).abs());
            }
        }
    };
    let at = |c: usize, r: usize| t.heights[r * t.w + c];
    walk(&|c| at(c, rect.r0), rect.c0, rect.c1);
    walk(&|c| at(c, rect.r1), rect.c0, rect.c1);
    walk(&|r| at(rect.c0, r), rect.r0, rect.r1);
    walk(&|r| at(rect.c1, r), rect.r0, rect.r1);
    err
}

/// Skirt depth, metres, for each LOD of a chunk: its own edge error at that LOD plus the worst at any LOD (what
/// a neighbour may show on the shared edge), plus `margin_m`.
pub fn skirt_depths(t: &Terrain, rect: ChunkRect, margin_m: f32) -> [f32; LOD_STRIDES.len()] {
    let errs = LOD_STRIDES.map(|s| edge_error(t, rect, s));
    let worst = errs.iter().copied().fold(0.0, f32::max);
    errs.map(|e| e + worst + margin_m)
}

/// Vertex data of one chunk mesh, local to the chunk's origin sample.
#[derive(Debug, Clone, Default)]
pub struct ChunkMesh {
    pub positions: Vec<[f32; 3]>,
    pub normals: Vec<[f32; 3]>,
    /// Linear RGBA.
    pub colors: Vec<[f32; 4]>,
    pub indices: Vec<u32>,
    /// Bounds of the surface (skirts included), local metres.
    pub min: [f32; 3],
    pub max: [f32; 3],
    /// Vertices of the surface grid (the rest are skirt vertices).
    pub grid_vertices: usize,
}

/// Builds the chunk at `stride` samples with skirts `skirt_m` deep (0 = none).
pub fn build_chunk(t: &Terrain, rect: ChunkRect, stride: usize, skirt_m: f32) -> ChunkMesh {
    let cols = axis(rect.c0, rect.c1, stride);
    let rows = axis(rect.r0, rect.r1, stride);
    let (nc, nr) = (cols.len(), rows.len());
    let mut m = ChunkMesh { min: [f32::INFINITY; 3], max: [f32::NEG_INFINITY; 3], ..ChunkMesh::default() };
    let capacity = nc * nr + 2 * (nc + nr);
    m.positions.reserve(capacity);
    m.normals.reserve(capacity);
    m.colors.reserve(capacity);
    for &r in &rows {
        for &c in &cols {
            let h = t.heights[r * t.w + c];
            let n = t.normal(c, r, stride);
            m.push([(c - rect.c0) as f32 * t.spacing, h, (r - rect.r0) as f32 * t.spacing], n, colour(t, (c as f32 * t.spacing, r as f32 * t.spacing), h, n[1]));
        }
    }
    m.grid_vertices = m.positions.len();
    m.indices.reserve((nc - 1) * (nr - 1) * 6 + 12 * (nc + nr));
    for j in 0..nr - 1 {
        for i in 0..nc - 1 {
            let a = (j * nc + i) as u32;
            let (b, c, d) = (a + 1, a + nc as u32, a + nc as u32 + 1);
            // Counter-clockwise seen from above (+y): normal (c - a) × (d - a) points up.
            m.indices.extend_from_slice(&[a, c, d, a, d, b]);
        }
    }
    if skirt_m > 0.0 {
        let north: Vec<usize> = (0..nc).collect();
        let south: Vec<usize> = (0..nc).map(|i| (nr - 1) * nc + i).collect();
        let west: Vec<usize> = (0..nr).map(|j| j * nc).collect();
        let east: Vec<usize> = (0..nr).map(|j| j * nc + nc - 1).collect();
        for (edge, outward) in [(north, [0.0, 0.0, -1.0]), (south, [0.0, 0.0, 1.0]), (west, [-1.0, 0.0, 0.0]), (east, [1.0, 0.0, 0.0])] {
            m.skirt(&edge, skirt_m, outward);
        }
    }
    m
}

impl ChunkMesh {
    /// Appends a vertex and grows the bounds.
    pub fn push(&mut self, p: [f32; 3], n: [f32; 3], colour: [f32; 4]) {
        for (k, v) in p.iter().enumerate() {
            self.min[k] = self.min[k].min(*v);
            self.max[k] = self.max[k].max(*v);
        }
        self.positions.push(p);
        self.normals.push(n);
        self.colors.push(colour);
    }

    /// A vertical strip hanging `depth` below the edge vertices `edge`, facing `outward`.
    fn skirt(&mut self, edge: &[usize], depth: f32, outward: [f32; 3]) {
        let base = self.positions.len() as u32;
        for &v in edge {
            let [x, y, z] = self.positions[v];
            let (n, colour) = (self.normals[v], self.colors[v]);
            self.push([x, y - depth, z], n, colour);
        }
        for k in 0..edge.len() - 1 {
            let (e0, e1) = (edge[k] as u32, edge[k + 1] as u32);
            let (s0, s1) = (base + k as u32, base + k as u32 + 1);
            for tri in [[e0, s0, s1], [e0, s1, e1]] {
                let tri = if dot(self.face_normal(tri), outward) >= 0.0 { tri } else { [tri[0], tri[2], tri[1]] };
                self.indices.extend_from_slice(&tri);
            }
        }
    }

    /// Unnormalised normal of a triangle by its winding: `(p1 - p0) × (p2 - p0)`.
    pub fn face_normal(&self, [i0, i1, i2]: [u32; 3]) -> [f32; 3] {
        let (p0, p1, p2) = (self.positions[i0 as usize], self.positions[i1 as usize], self.positions[i2 as usize]);
        cross(sub(p1, p0), sub(p2, p0))
    }
}

// Viewer palette, sRGB (a neutral look-test ramp, not a game material).
const LOWLAND: [f32; 3] = [0.40, 0.41, 0.26];
const SUMMIT: [f32; 3] = [0.78, 0.76, 0.72];
const ROCK: [f32; 3] = [0.46, 0.44, 0.42];
const SEABED: [f32; 3] = [0.30, 0.30, 0.26];
/// Multiplier toward dry, browner ground (the tint's second noise).
const DRY: [f32; 3] = [1.10, 0.97, 0.80];

/// Vertex colour (linear RGBA) at terrain-local metres `xz`, height `h`, normal y `ny`: olive low ground, lighter
/// toward the summit, rock grey on slopes steeper than ~25-40°, then [`Tint`]: two octaves of value noise vary
/// the brightness and, independently, shift toward a drier brown.
pub fn colour(t: &Terrain, (x, z): (f32, f32), h: f32, ny: f32) -> [f32; 4] {
    let base = if h < t.sea_level {
        SEABED
    } else {
        let alt = (h - t.sea_level) / (t.max_h - t.sea_level).max(1.0);
        mix(LOWLAND, SUMMIT, smoothstep(0.45, 0.95, alt))
    };
    // cos 25° = 0.906, cos 40° = 0.766.
    let rock = 1.0 - smoothstep(0.766, 0.906, ny);
    let mut srgb = mix(base, ROCK, rock);
    if t.tint.amount > 0.0 && t.tint.wavelength_m > 0.0 {
        let octaves = |seed: u32| {
            let l = t.tint.wavelength_m;
            0.65 * value_noise(x / l, z / l, seed) + 0.35 * value_noise(4.0 * x / l, 4.0 * z / l, seed + 1)
        };
        let bright = 1.0 + t.tint.amount * (2.0 * octaves(11) - 1.0);
        let dry = t.tint.amount * 2.0 * octaves(23);
        srgb = mix(srgb, mul(srgb, DRY), dry.min(1.0)).map(|ch| (ch * bright).clamp(0.0, 1.0));
    }
    let [r, g, b] = srgb.map(srgb_to_linear);
    [r, g, b, 1.0]
}

fn srgb_to_linear(c: f32) -> f32 {
    if c <= 0.04045 { c / 12.92 } else { ((c + 0.055) / 1.055).powf(2.4) }
}

fn smoothstep(a: f32, b: f32, x: f32) -> f32 {
    let t = ((x - a) / (b - a)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

fn mix(a: [f32; 3], b: [f32; 3], t: f32) -> [f32; 3] {
    [lerp(a[0], b[0], t), lerp(a[1], b[1], t), lerp(a[2], b[2], t)]
}

fn mul(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    [a[0] * b[0], a[1] * b[1], a[2] * b[2]]
}

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

fn sub(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

fn cross(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]
}

fn dot(a: [f32; 3], b: [f32; 3]) -> f32 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

fn normalize(v: [f32; 3]) -> [f32; 3] {
    let len = dot(v, v).sqrt();
    [v[0] / len, v[1] / len, v[2] / len]
}
