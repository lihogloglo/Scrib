//! CPU height queries over resident tiles (physics, AI, foliage, water).
//!
//! Reads base + persistent edits; transient deformation and detail
//! displacement are render-only (terrain.md). A query uses the finest
//! resident level at the point, so it answers anywhere a coarse level is
//! resident. The surface is the canonical one: quads split along
//! [`CANONICAL_DIAGONAL`], the same triangles colliders and the render
//! patch use.

use std::collections::HashMap;
use std::sync::Arc;

use glam::{DVec2, DVec3, Vec2, Vec3};
use sf_coords::WorldPos;

use crate::edits::HeightEdits;
use crate::pyramid::BlockBounds;
use crate::source::{floor_i64, triangle_height};
use crate::tile::{BLOCK_QUADS, BLOCKS, CANONICAL_DIAGONAL, HeightTile, QUADS_F64, TILE_QUADS, TileCoord, global_coords};

/// A ray in one worldspace. `dir` must be normalized.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WorldRay {
    pub origin: WorldPos,
    pub dir: Vec3,
}

/// An XZ rectangle; Y of `min` is ignored.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WorldRect {
    pub min: WorldPos,
    pub size: Vec2,
}

impl WorldRect {
    /// Samples per axis at `spacing`: both edges included.
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    pub fn sample_count(&self, spacing: f32) -> (usize, usize) {
        let n = |len: f32| (len / spacing).round().max(0.0) as usize + 1;
        (n(self.size.x), n(self.size.y))
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TerrainHit {
    pub pos: WorldPos,
    /// Face normal of the triangle hit, pointing up.
    pub normal: Vec3,
    /// Distance along the ray in meters.
    pub distance: f32,
}

/// A decoded tile ready for queries.
#[derive(Clone, Debug, PartialEq)]
pub struct ResidentTile {
    pub tile: HeightTile,
    pub bounds: BlockBounds,
}

impl ResidentTile {
    pub fn new(tile: HeightTile) -> Self {
        let bounds = BlockBounds::of(&tile);
        Self { tile, bounds }
    }
}

/// Resident terrain of one worldspace: base tiles (any levels) plus edits.
#[derive(Clone, Default)]
pub struct TerrainHeights {
    tiles: HashMap<TileCoord, Arc<ResidentTile>>,
    /// Number of levels to probe (highest resident level + 1).
    levels: u8,
    edits: HeightEdits,
}

impl TerrainHeights {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, tile: Arc<ResidentTile>) {
        let coord = tile.tile.coord;
        self.levels = self.levels.max(coord.level + 1);
        self.tiles.insert(coord, tile);
    }

    pub fn remove(&mut self, coord: TileCoord) -> Option<Arc<ResidentTile>> {
        self.tiles.remove(&coord)
    }

    pub fn get(&self, coord: TileCoord) -> Option<&Arc<ResidentTile>> {
        self.tiles.get(&coord)
    }

    pub fn edits(&self) -> &HeightEdits {
        &self.edits
    }

    /// Edit through the returned layer; its changes report `TerrainChanged`.
    pub fn edits_mut(&mut self) -> &mut HeightEdits {
        &mut self.edits
    }

    /// Terrain height under `p` (its XZ), or `None` if nothing is resident.
    pub fn height(&self, p: WorldPos) -> Option<f32> {
        self.height_at(global_coords(p))
    }

    /// Smooth normal: central differences one sample apart at the resolved
    /// level (what the shader does).
    pub fn normal(&self, p: WorldPos) -> Option<Vec3> {
        let g = global_coords(p);
        let (tile, _) = self.locate(g)?;
        let e = f64::from(1u32 << tile.tile.coord.level);
        let h = self.height_at(g)?;
        let at = |dx: f64, dz: f64| self.height_at(g + DVec2::new(dx, dz)).unwrap_or(h);
        // e level-0 samples are e/2 meters.
        #[allow(clippy::cast_possible_truncation)]
        let span = e as f32;
        let slope_x = (at(e, 0.0) - at(-e, 0.0)) / span;
        let slope_z = (at(0.0, e) - at(0.0, -e)) / span;
        Some(Vec3::new(-slope_x, 1.0, -slope_z).normalize())
    }

    /// Heights on a grid over `rect`, `spacing` meters apart, row-major
    /// (x fastest). `out` must hold `rect.sample_count(spacing)` samples.
    /// Non-resident samples are NaN; returns how many there were.
    pub fn sample_rect(&self, rect: WorldRect, spacing: f32, out: &mut [f32]) -> usize {
        let (nx, nz) = rect.sample_count(spacing);
        assert_eq!(out.len(), nx * nz, "sample_rect: `out` has the wrong length");
        let g0 = global_coords(rect.min);
        let step = f64::from(spacing) * 2.0;
        let mut missing = 0;
        for (j, row) in out.chunks_exact_mut(nx).enumerate() {
            #[allow(clippy::cast_precision_loss)]
            let gz = g0.y + j as f64 * step;
            for (i, h) in row.iter_mut().enumerate() {
                #[allow(clippy::cast_precision_loss)]
                let gx = g0.x + i as f64 * step;
                *h = self.height_at(DVec2::new(gx, gz)).unwrap_or_else(|| {
                    missing += 1;
                    f32::NAN
                });
            }
        }
        missing
    }

    /// First crossing of the surface along `ray` within `max` meters (from
    /// either side), over resident tiles. Non-resident stretches are skipped.
    pub fn raycast(&self, ray: WorldRay, max: f32) -> Option<TerrainHit> {
        let o = global_coords(ray.origin);
        let oy = ray.origin.to_f64().y;
        let dir = ray.dir.as_dvec3();
        // Level-0 sample units per meter of travel.
        let d = DVec2::new(dir.x, dir.z) * 2.0;
        let max = f64::from(max);
        let mut t = 0.0;
        while t <= max {
            let g = o + d * t;
            let tile = self.locate(g).map(|(tile, _)| tile);
            // Without a resident tile, skip to the next level-0 tile.
            let coord = tile.map_or_else(
                || {
                    let c = |v: f64| i32::try_from(floor_i64(v / QUADS_F64)).unwrap_or(i32::MAX);
                    TileCoord::new(0, c(g.x), c(g.y))
                },
                |t| t.tile.coord,
            );
            let exit = tile_exit(coord, o, d, t).min(max);
            if let Some(tile) = tile
                && let Some(hit) = self.intersect_tile(tile, o, oy, dir, t, exit)
            {
                #[allow(clippy::cast_possible_truncation)]
                return Some(TerrainHit {
                    pos: ray.origin.offset((dir * hit.0).as_vec3()),
                    normal: hit.1,
                    distance: hit.0 as f32,
                });
            }
            // Step just past the border into the next tile.
            t = exit + 1e-4;
        }
        None
    }

    /// The finest resident tile containing continuous global coordinates
    /// `g`, and `g` in that tile's sample units.
    fn locate(&self, g: DVec2) -> Option<(&ResidentTile, DVec2)> {
        (0..self.levels).find_map(|level| {
            let q = QUADS_F64 * f64::from(1u32 << level);
            let (tx, tz) = (floor_i64(g.x / q), floor_i64(g.y / q));
            let coord = TileCoord::new(level, i32::try_from(tx).ok()?, i32::try_from(tz).ok()?);
            let tile = self.tiles.get(&coord)?;
            #[allow(clippy::cast_precision_loss)]
            let local = (g - DVec2::new(tx as f64, tz as f64) * q) / f64::from(1u32 << level);
            Some((&**tile, local))
        })
    }

    /// Base + edits at sample (i, j) of `tile`.
    fn sample(&self, tile: &ResidentTile, i: usize, j: usize) -> f32 {
        let base = tile.tile.get(i, j);
        if self.edits.is_empty() {
            return base;
        }
        let (gx, gz) = tile.tile.coord.global_sample(i, j);
        base + self.edits.delta(gx, gz)
    }

    fn corners(&self, tile: &ResidentTile, i: usize, j: usize) -> [f32; 4] {
        [self.sample(tile, i, j), self.sample(tile, i + 1, j), self.sample(tile, i, j + 1), self.sample(tile, i + 1, j + 1)]
    }

    fn height_at(&self, g: DVec2) -> Option<f32> {
        let (tile, local) = self.locate(g)?;
        let (i, fx) = cell(local.x);
        let (j, fz) = cell(local.y);
        Some(triangle_height(CANONICAL_DIAGONAL, self.corners(tile, i, j), fx, fz))
    }

    /// Ray against one tile's triangles for t in [t0, t1]: a DDA over the
    /// 16×16 blocks, skipping blocks the ray passes above or below, then a
    /// DDA over the quads of the blocks it may hit.
    fn intersect_tile(
        &self,
        tile: &ResidentTile,
        o: DVec2,
        oy: f64,
        dir: DVec3,
        t0: f64,
        t1: f64,
    ) -> Option<(f64, Vec3)> {
        let coord = tile.tile.coord;
        let scale = f64::from(1u32 << coord.level);
        let (gx0, gz0) = coord.first_sample();
        #[allow(clippy::cast_precision_loss)]
        let origin = DVec2::new(gx0 as f64, gz0 as f64);
        // Ray in tile sample units.
        let p0 = (o - origin) / scale;
        let d = DVec2::new(dir.x, dir.z) * 2.0 / scale;
        let cull = !self.edits.touches(coord);
        let block = f64::from(u32::try_from(BLOCK_QUADS).expect("small"));
        let mut hit = None;
        walk(p0 / block, d / block, t0, t1, BLOCKS, |bx, bz, ta, tb| {
            if cull {
                let (lo, hi) = tile.bounds.get(bx, bz);
                let (ya, yb) = (oy + dir.y * ta, oy + dir.y * tb);
                if ya.min(yb) > f64::from(hi) || ya.max(yb) < f64::from(lo) {
                    return false;
                }
            }
            walk(p0, d, ta, tb, TILE_QUADS, |i, j, qa, qb| {
                hit = self.intersect_quad(tile, i, j, (o, oy, dir), qa, qb);
                hit.is_some()
            })
        });
        hit
    }

    /// Ray against the two triangles of quad (i, j), accepting t in
    /// [ta, tb] (plus 1 mm, so hits on a cell border aren't lost).
    fn intersect_quad(
        &self,
        tile: &ResidentTile,
        i: usize,
        j: usize,
        (o, oy, dir): (DVec2, f64, DVec3),
        ta: f64,
        tb: f64,
    ) -> Option<(f64, Vec3)> {
        let coord = tile.tile.coord;
        let [h00, h10, h01, h11] = self.corners(tile, i, j);
        // Corners relative to the ray origin, in meters.
        let corner = |a: usize, b: usize, h: f32| {
            let (gx, gz) = coord.global_sample(a, b);
            #[allow(clippy::cast_precision_loss)]
            let g = DVec2::new(gx as f64, gz as f64);
            let rel = (g - o) * 0.5;
            DVec3::new(rel.x, f64::from(h) - oy, rel.y)
        };
        let v00 = corner(i, j, h00);
        let v10 = corner(i + 1, j, h10);
        let v01 = corner(i, j + 1, h01);
        let v11 = corner(i + 1, j + 1, h11);
        // CANONICAL_DIAGONAL is Main: triangles (00, 10, 11) and (00, 11, 01).
        let tris = [[v00, v10, v11], [v00, v11, v01]];
        let mut best: Option<(f64, Vec3)> = None;
        for [a, b, c] in tris {
            if let Some(t) = ray_triangle(dir, a, b, c)
                && t >= ta - 1e-3
                && t <= tb + 1e-3
                && best.is_none_or(|(bt, _)| t < bt)
            {
                let mut n = (b - a).cross(c - a).normalize();
                if n.y < 0.0 {
                    n = -n;
                }
                best = Some((t, n.as_vec3()));
            }
        }
        best
    }
}

/// Splits a tile-local coordinate into a quad index in [0, 511] and a
/// fraction in [0, 1].
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::cast_precision_loss)]
fn cell(v: f64) -> (usize, f32) {
    let i = (v.floor().max(0.0) as usize).min(TILE_QUADS - 1);
    (i, (v - i as f64).clamp(0.0, 1.0) as f32)
}

/// Ray parameter where the ray (at `t` inside `coord`) leaves the tile.
fn tile_exit(coord: TileCoord, o: DVec2, d: DVec2, t: f64) -> f64 {
    let q = QUADS_F64 * f64::from(1u32 << coord.level);
    let lo = DVec2::new(f64::from(coord.x), f64::from(coord.z)) * q;
    let hi = lo + DVec2::splat(q);
    let mut exit = f64::INFINITY;
    for a in 0..2 {
        if d[a] > 0.0 {
            exit = exit.min((hi[a] - o[a]) / d[a]);
        } else if d[a] < 0.0 {
            exit = exit.min((lo[a] - o[a]) / d[a]);
        }
    }
    exit.max(t)
}

/// Amanatides–Woo grid traversal: visits the unit cells of an `n`×`n` grid
/// crossed by `p0 + d·t` for t in [t0, t1], in order, with each cell's
/// t-interval. Stops (returning true) when `f` returns true.
#[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation, clippy::cast_sign_loss)]
fn walk(p0: DVec2, d: DVec2, t0: f64, t1: f64, n: usize, mut f: impl FnMut(usize, usize, f64, f64) -> bool) -> bool {
    let last = i64::try_from(n).expect("small grid") - 1;
    let start = p0 + d * t0;
    let mut c = [floor_i64(start.x).clamp(0, last), floor_i64(start.y).clamp(0, last)];
    let mut step = [0i64; 2];
    let mut next = [f64::INFINITY; 2];
    let mut delta = [f64::INFINITY; 2];
    for a in 0..2 {
        if d[a] > 0.0 {
            step[a] = 1;
            next[a] = ((c[a] + 1) as f64 - p0[a]) / d[a];
            delta[a] = 1.0 / d[a];
        } else if d[a] < 0.0 {
            step[a] = -1;
            next[a] = (c[a] as f64 - p0[a]) / d[a];
            delta[a] = -1.0 / d[a];
        }
    }
    let mut ta = t0;
    loop {
        let a = usize::from(next[1] < next[0]);
        let tb = next[a].min(t1);
        if f(c[0] as usize, c[1] as usize, ta, tb) {
            return true;
        }
        if tb >= t1 {
            return false;
        }
        c[a] += step[a];
        if c[a] < 0 || c[a] > last {
            return false;
        }
        ta = tb;
        next[a] += delta[a];
    }
}

/// Möller–Trumbore, two-sided, ray from the origin along `dir`.
fn ray_triangle(dir: DVec3, a: DVec3, b: DVec3, c: DVec3) -> Option<f64> {
    let e1 = b - a;
    let e2 = c - a;
    let p = dir.cross(e2);
    let det = e1.dot(p);
    if det.abs() < 1e-12 {
        return None;
    }
    let inv = 1.0 / det;
    let s = -a;
    let u = s.dot(p) * inv;
    if !(-1e-9..=1.0 + 1e-9).contains(&u) {
        return None;
    }
    let q = s.cross(e1);
    let v = dir.dot(q) * inv;
    if v < -1e-9 || u + v > 1.0 + 1e-9 {
        return None;
    }
    let t = e2.dot(q) * inv;
    (t >= 0.0).then_some(t)
}
