//! Scale references for `worldgen-view` (slice WG-V2), Bevy-free: town markers from `markers.json` at their
//! scaled positions but hand-made size, deterministic tree cones, the contour-line formula, compass bearings.
//!
//! Frames: a marker is in **source metres** (`mw_worldgen::markers`, x east, y north). An arm generated at scale
//! S (S2/S2b arms: the guide frame × 10, their `README.md` "World frame = guide frame x S") holds it at
//! `S × p` in its own frame, whose sample (0, 0) is the arm's `meta.json` `origin_m`. Terrain-local metres (the
//! viewer's mesh frame, [`super::mesh`]) are then `x = S·p.x − origin.x`, `z = origin.y − S·p.y` (z south).
//! Only the position scales: a town keeps its source cells (117 m) around its scaled centroid.

#![allow(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_possible_wrap,
    clippy::many_single_char_names,
    reason = "grid indices, hash bits and metres shown in f32; x, y, z, r, h are the geometry's names"
)]

use mw_worldgen::{Marker, SummitMarker};

use super::mesh::{ChunkMesh, ChunkRect, Terrain};

// ---------------------------------------------------------------------------------------------------------------
// Hashing and noise.

/// A 32-bit integer hash of a lattice point (lowbias32 finaliser over a mixed key): deterministic, platform-free.
pub fn hash2(x: i32, y: i32, seed: u32) -> u32 {
    let mut h = (x as u32).wrapping_mul(0x8da6_b343) ^ (y as u32).wrapping_mul(0xd816_3841) ^ seed.wrapping_mul(0xcb1a_b31f);
    h ^= h >> 16;
    h = h.wrapping_mul(0x7feb_352d);
    h ^= h >> 15;
    h = h.wrapping_mul(0x846c_a68b);
    h ^ (h >> 16)
}

/// The top 24 bits of `h` as a number in `[0, 1)`.
pub fn unit(h: u32) -> f32 {
    (h >> 8) as f32 / (1u32 << 24) as f32
}

/// Value noise in `[0, 1)` at lattice coordinates `(x, y)`: hashed corners, smoothstep-interpolated.
pub fn value_noise(x: f32, y: f32, seed: u32) -> f32 {
    let (fx, fy) = (x.floor(), y.floor());
    let (ix, iy) = (fx as i32, fy as i32);
    let s = |t: f32| t * t * (3.0 - 2.0 * t);
    let (tx, ty) = (s(x - fx), s(y - fy));
    let corner = |dx: i32, dy: i32| unit(hash2(ix.wrapping_add(dx), iy.wrapping_add(dy), seed));
    let top = corner(0, 0) + (corner(1, 0) - corner(0, 0)) * tx;
    let bottom = corner(0, 1) + (corner(1, 1) - corner(0, 1)) * tx;
    top + (bottom - top) * ty
}

// ---------------------------------------------------------------------------------------------------------------
// Trees.

/// Tree scatter settings (the `--tree-*` flags).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TreeParams {
    /// Candidate grid spacing, metres: at most one tree per cell.
    pub spacing_m: f32,
    /// Share of candidates kept, 0..1 (0: no trees).
    pub density: f32,
    /// Cone height range, metres.
    pub height_m: [f32; 2],
    /// Base radius per metre of height.
    pub radius_ratio: f32,
    /// Steepest ground with trees, degrees.
    pub max_slope_deg: f32,
    /// Highest ground with trees, metres above the sea (a treeline).
    pub max_above_sea_m: f32,
    /// Forest-patch noise wavelength, metres (0: uniform density).
    pub clump_m: f32,
}

/// One cone, terrain-local metres.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Tree {
    /// Base centre on the ground (x, ground height, z).
    pub pos: [f32; 3],
    pub height: f32,
    pub radius: f32,
    /// Brightness factor of its colour.
    pub shade: f32,
}

/// Sides of a tree cone (mesh layout).
// tunables: const (mesh layout of a scale-cue cone)
pub const TREE_SIDES: usize = 6;
/// Lowest ground above the sea with trees, metres (keeps them off the beach line).
// tunables: const (keeps cones out of the sea plane; not a look setting)
const TREE_SHORE_M: f32 = 1.0;

/// Trees whose base falls inside chunk `rect` (half-open, so neighbours never both hold one) of an arm whose sample
/// (0, 0) is at arm-frame metres `origin_m`. Candidates sit on a grid in the **arm frame** and are hashed by
/// their integer cell, so a tree is where it is whatever the chunking or the loading order; it is kept on
/// gentle, dry ground below the treeline.
pub fn scatter_trees(t: &Terrain, rect: ChunkRect, origin_m: [f64; 2], p: &TreeParams) -> Vec<Tree> {
    let mut out = Vec::new();
    if p.density <= 0.0 || p.spacing_m <= 0.0 {
        return out;
    }
    let s = f64::from(p.spacing_m);
    let sp = f64::from(t.spacing);
    let (x0, x1) = (rect.c0 as f64 * sp, rect.c1 as f64 * sp);
    let (z0, z1) = (rect.r0 as f64 * sp, rect.r1 as f64 * sp);
    let [ox, oy] = origin_m;
    let (i0, i1) = (((ox + x0) / s).floor() as i32, ((ox + x1) / s).floor() as i32);
    let (j0, j1) = (((oy - z1) / s).floor() as i32, ((oy - z0) / s).floor() as i32);
    let min_ny = p.max_slope_deg.to_radians().cos();
    for j in j0..=j1 {
        for i in i0..=i1 {
            let ax = (f64::from(i) + f64::from(unit(hash2(i, j, 1)))) * s;
            let ay = (f64::from(j) + f64::from(unit(hash2(i, j, 2)))) * s;
            let (x, z) = (ax - ox, oy - ay);
            if !(x >= x0 && x < x1 && z >= z0 && z < z1) {
                continue;
            }
            let mut keep = p.density;
            if p.clump_m > 0.0 {
                let c = f64::from(p.clump_m);
                let n = value_noise((ax / c) as f32, (ay / c) as f32, 7);
                keep *= smoothstep(0.3, 0.7, n);
            }
            if unit(hash2(i, j, 3)) >= keep {
                continue;
            }
            let (x, z) = (x as f32, z as f32);
            let Some(ground) = t.height_at(x, z) else { continue };
            let above = ground - t.sea_level;
            if above < TREE_SHORE_M || above > p.max_above_sea_m {
                continue;
            }
            let c = ((x / t.spacing).round() as usize).min(t.w - 1);
            let r = ((z / t.spacing).round() as usize).min(t.h - 1);
            if t.normal(c, r, 1)[1] < min_ny {
                continue;
            }
            let [lo, hi] = p.height_m;
            let height = lo + (hi - lo) * unit(hash2(i, j, 4));
            let shade = 0.8 + 0.4 * unit(hash2(i, j, 5));
            out.push(Tree { pos: [x, ground, z], height, radius: height * p.radius_ratio, shade });
        }
    }
    out
}

// Conifer green, sRGB.
const TREE_GREEN: [f32; 3] = [0.12, 0.21, 0.09];

/// One mesh of cones, local to `origin` (terrain-local metres of the chunk's first sample). Each cone's base sinks
/// `radius · tan(max_slope)` below its ground point, so it never floats on a slope the scatter allows.
pub fn tree_mesh(trees: &[Tree], origin: [f32; 2], max_slope_deg: f32) -> ChunkMesh {
    let mut m = ChunkMesh { min: [f32::INFINITY; 3], max: [f32::NEG_INFINITY; 3], ..ChunkMesh::default() };
    let sink_per_m = max_slope_deg.to_radians().tan();
    for tree in trees {
        let [x, y, z] = tree.pos;
        let (cx, cz) = (x - origin[0], z - origin[1]);
        let base = y - tree.radius * sink_per_m;
        let apex = y + tree.height;
        let (r, h) = (tree.radius, apex - base);
        let colour = srgb_linear(TREE_GREEN.map(|c| (c * tree.shade).min(1.0)));
        let side = |theta: f32| {
            let (s, c) = theta.sin_cos();
            normalise([h * c, r, h * s])
        };
        let first = m.positions.len() as u32;
        for k in 0..TREE_SIDES {
            let theta = k as f32 / TREE_SIDES as f32 * std::f32::consts::TAU;
            m.push([cx + r * theta.cos(), base, cz + r * theta.sin()], side(theta), colour);
        }
        for k in 0..TREE_SIDES {
            let theta = (k as f32 + 0.5) / TREE_SIDES as f32 * std::f32::consts::TAU;
            m.push([cx, apex, cz], side(theta), colour);
        }
        for k in 0..TREE_SIDES {
            let (b0, b1) = (first + k as u32, first + ((k + 1) % TREE_SIDES) as u32);
            let a = first + (TREE_SIDES + k) as u32;
            let theta = (k as f32 + 0.5) / TREE_SIDES as f32 * std::f32::consts::TAU;
            let outward = [theta.cos(), 0.0, theta.sin()];
            let tri = [b0, b1, a];
            let tri = if dot(m.face_normal(tri), outward) >= 0.0 { tri } else { [b0, a, b1] };
            m.indices.extend_from_slice(&tri);
        }
    }
    m.grid_vertices = m.positions.len();
    m
}

// ---------------------------------------------------------------------------------------------------------------
// Markers.

/// Source metres → terrain-local metres (x east, z south of sample (0, 0)) of an arm at `scale` whose sample (0, 0)
/// is at arm-frame metres `origin_m` (module docs).
pub fn source_to_local(p: [f64; 2], scale: f64, origin_m: [f64; 2]) -> (f32, f32) {
    ((p[0] * scale - origin_m[0]) as f32, (origin_m[1] - p[1] * scale) as f32)
}

/// A box standing on the terrain, terrain-local metres.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Block {
    /// Footprint centre (x, z).
    pub centre: (f32, f32),
    /// Half the footprint edge.
    pub half: f32,
    /// Bottom (below the lowest ground under it) and top heights.
    pub base: f32,
    pub top: f32,
}

/// A marker placed on one arm.
#[derive(Debug, Clone, PartialEq)]
pub struct Placed {
    pub name: String,
    pub cells: usize,
    /// The scaled centroid (x, z), terrain-local metres.
    pub at: (f32, f32),
    /// Height of the label anchor: the highest block top.
    pub top: f32,
    pub blocks: Vec<Block>,
}

/// How far a block's bottom goes under the lowest ground it covers, metres (no gap on uneven ground).
// tunables: const (geometry fudge against a visible gap; not a look setting)
const BLOCK_SINK_M: f32 = 1.0;

/// A box of edge `2·half` at local `centre`, standing on `t`: from under its lowest ground (corners and centre)
/// to `tall` above its highest. `None` when any of those points is off the grid.
pub fn block_on(t: &Terrain, centre: (f32, f32), half: f32, tall: f32) -> Option<Block> {
    let (x, z) = centre;
    let mut lo = f32::INFINITY;
    let mut hi = f32::NEG_INFINITY;
    for (dx, dz) in [(0.0, 0.0), (-half, -half), (half, -half), (-half, half), (half, half)] {
        let g = t.height_at(x + dx, z + dz)?.max(t.sea_level);
        lo = lo.min(g);
        hi = hi.max(g);
    }
    Some(Block { centre, half, base: lo - BLOCK_SINK_M, top: hi + tall })
}

/// `marker` on `t`: one block per source cell (`cell_m` edge, at its true offset from the centroid, unscaled)
/// around the centroid at `scale`. `None` when the centroid is off the grid or under the sea (another arm's
/// coverage, or Solstheim masked to sea); cells off the grid are dropped.
pub fn place_marker(m: &Marker, t: &Terrain, scale: f64, origin_m: [f64; 2], cell_m: f64, tall: f32) -> Option<Placed> {
    let at = source_to_local(m.centroid_m, scale, origin_m);
    if t.height_at(at.0, at.1)? < t.sea_level {
        return None;
    }
    let half = (cell_m / 2.0) as f32;
    let blocks: Vec<Block> = m
        .cells
        .iter()
        .filter_map(|&[gx, gy]| {
            let c = mw_worldgen::markers::cell_centre_m((gx, gy));
            // 1:1 offset from the centroid; north (+y) is −z.
            let centre = (at.0 + (c[0] - m.centroid_m[0]) as f32, at.1 - (c[1] - m.centroid_m[1]) as f32);
            block_on(t, centre, half, tall)
        })
        .collect();
    let top = blocks.iter().map(|b| b.top).fold(f32::NEG_INFINITY, f32::max);
    (!blocks.is_empty()).then(|| Placed { name: m.name.clone(), cells: m.cells.len(), at, top, blocks })
}

/// The source summit on `t` as a thin pole (`half` wide, `tall` above the ground there).
pub fn place_summit(s: &SummitMarker, t: &Terrain, scale: f64, origin_m: [f64; 2], half: f32, tall: f32) -> Option<Placed> {
    let at = source_to_local([s.x_m, s.y_m], scale, origin_m);
    let block = block_on(t, at, half, tall)?;
    Some(Placed {
        name: format!("source summit ({:.0} m x{scale})", s.height_m),
        cells: 0,
        at,
        top: block.top,
        blocks: vec![block],
    })
}

/// Boxes as one flat-shaded mesh (24 vertices each), local to `origin` (terrain-local x, z), colour linear RGBA.
pub fn blocks_mesh(blocks: &[Block], origin: [f32; 2], colour: [f32; 4]) -> ChunkMesh {
    let mut m = ChunkMesh { min: [f32::INFINITY; 3], max: [f32::NEG_INFINITY; 3], ..ChunkMesh::default() };
    for b in blocks {
        let (x0, x1) = (b.centre.0 - b.half - origin[0], b.centre.0 + b.half - origin[0]);
        let (z0, z1) = (b.centre.1 - b.half - origin[1], b.centre.1 + b.half - origin[1]);
        let (y0, y1) = (b.base, b.top);
        // (normal, four corners counter-clockwise seen from outside).
        let faces: [([f32; 3], [[f32; 3]; 4]); 6] = [
            ([0.0, 1.0, 0.0], [[x0, y1, z0], [x0, y1, z1], [x1, y1, z1], [x1, y1, z0]]),
            ([0.0, -1.0, 0.0], [[x0, y0, z0], [x1, y0, z0], [x1, y0, z1], [x0, y0, z1]]),
            ([1.0, 0.0, 0.0], [[x1, y0, z0], [x1, y1, z0], [x1, y1, z1], [x1, y0, z1]]),
            ([-1.0, 0.0, 0.0], [[x0, y0, z0], [x0, y0, z1], [x0, y1, z1], [x0, y1, z0]]),
            ([0.0, 0.0, 1.0], [[x0, y0, z1], [x1, y0, z1], [x1, y1, z1], [x0, y1, z1]]),
            ([0.0, 0.0, -1.0], [[x0, y0, z0], [x0, y1, z0], [x1, y1, z0], [x1, y0, z0]]),
        ];
        for (n, corners) in faces {
            let first = m.positions.len() as u32;
            for p in corners {
                m.push(p, n, colour);
            }
            m.indices.extend_from_slice(&[first, first + 1, first + 2, first, first + 2, first + 3]);
        }
    }
    m.grid_vertices = m.positions.len();
    m
}

/// The nearest of `markers` to local `(x, z)` by horizontal distance: (index, metres).
pub fn nearest(markers: &[Placed], (x, z): (f32, f32)) -> Option<(usize, f32)> {
    markers
        .iter()
        .enumerate()
        .map(|(i, m)| (i, (m.at.0 - x).hypot(m.at.1 - z)))
        .min_by(|a, b| a.1.total_cmp(&b.1))
}

/// Eight-point compass bearing of a horizontal offset (x east, z south).
pub fn compass(dx: f32, dz: f32) -> &'static str {
    const POINTS: [&str; 8] = ["N", "NE", "E", "SE", "S", "SW", "W", "NW"];
    // Clockwise from north: north is −z.
    let deg = dx.atan2(-dz).to_degrees().rem_euclid(360.0);
    POINTS[((deg + 22.5) / 45.0) as usize % 8]
}

// ---------------------------------------------------------------------------------------------------------------
// Contours.

/// Contour line settings (the `--contour-*` flags); the GPU uniform of `terrain.wesl`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Contours {
    /// Minor and major intervals, metres.
    pub interval_m: [f32; 2],
    /// Minor and major line widths, pixels.
    pub width_px: [f32; 2],
}

/// Ink (0..1) of the contour lines at height `h` where the height changes by `dh_per_px` metres per pixel: the CPU
/// mirror of `terrain.wesl`'s `contour_ink` (keep them identical). Lines are a constant `width_px` wide on screen;
/// minor lines are drawn at 60%.
pub fn contour_ink(h: f32, dh_per_px: f32, c: &Contours) -> f32 {
    let dh = dh_per_px.max(1e-4);
    let line = |step: f32, width: f32| {
        if step <= 0.0 {
            return 0.0;
        }
        // WGSL `fract` is `x - floor(x)` (Rust's keeps the sign).
        let f = h / step + 0.5;
        let d = ((f - f.floor()) - 0.5).abs() * step / dh;
        (0.5 * width + 0.5 - d).clamp(0.0, 1.0)
    };
    (line(c.interval_m[0], c.width_px[0]) * 0.6).max(line(c.interval_m[1], c.width_px[1]))
}

// ---------------------------------------------------------------------------------------------------------------

fn smoothstep(a: f32, b: f32, x: f32) -> f32 {
    let t = ((x - a) / (b - a)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

fn dot(a: [f32; 3], b: [f32; 3]) -> f32 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

fn normalise(v: [f32; 3]) -> [f32; 3] {
    let len = dot(v, v).sqrt();
    [v[0] / len, v[1] / len, v[2] / len]
}

/// sRGB → linear RGBA (opaque).
pub fn srgb_linear(c: [f32; 3]) -> [f32; 4] {
    let f = |c: f32| if c <= 0.04045 { c / 12.92 } else { ((c + 0.055) / 1.055).powf(2.4) };
    [f(c[0]), f(c[1]), f(c[2]), 1.0]
}
